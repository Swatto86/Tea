//! Awake - System Tray Sleep Prevention Utility
//!
//! Prevents system sleep via F15 key simulation with optional display control.
//!
//! ## Architecture
//! - Core: Pure business logic (screen_mode, tooltip)
//! - Persistence: State file I/O
//! - Platform: OS-specific abstractions (Windows display control)
//! - Wake Service: Background task for input simulation
//! - UI: Tauri setup and menu event handling (this file)
//!
//! ## Design Principles
//! - Explicit errors, no unwrap/expect in production paths
//! - Separation of concerns (UI, business logic, platform code)
//! - Deliberate logging for debugging
//! - Side effects documented and isolated

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(warnings)]

mod core;
mod error;
mod icon;
mod persistence;
mod platform;
mod wake_service;

use crate::core::{ScreenMode, TooltipText};
use crate::persistence::{read_state, write_state, AppState};
use crate::wake_service::WakeService;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{image::Image, menu::{MenuBuilder, MenuId, MenuItemBuilder}, tray::TrayIconBuilder, Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Awake application");

    // Load persisted state
    let state = read_state();
    log::info!(
        "Loaded state: sleep_disabled={}, screen_mode={:?}",
        state.sleep_disabled,
        state.screen_mode
    );

    // Shared state for wake control
    let is_awake = Arc::new(AtomicBool::new(state.sleep_disabled));
    let screen_mode = Arc::new(Mutex::new(state.screen_mode));

    // Clone for Tauri builder closure
    let is_awake_clone = is_awake.clone();
    let screen_mode_clone = screen_mode.clone();
    let initial_state = state;

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(move |app| {
            setup_tray(app, initial_state, is_awake_clone, screen_mode_clone)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Setup system tray icon and menu
///
/// ## Design Intent
/// Configures UI layer - menu items, event handlers, initial state display.
/// Contains no business logic, only UI rendering and event delegation.
///
/// ## Arguments
/// * `app` - Tauri application handle
/// * `state` - Initial application state
/// * `is_awake` - Shared flag for wake state
/// * `screen_mode` - Shared screen mode preference
///
/// ## Side Effects
/// - Creates tray icon
/// - Registers menu event handlers
/// - May start wake service if state.sleep_disabled is true
///
/// ## Returns
/// Ok(()) on success, or error if tray setup fails
fn setup_tray(
    app: &mut tauri::App,
    state: AppState,
    is_awake: Arc<AtomicBool>,
    screen_mode: Arc<Mutex<ScreenMode>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    // Menu item IDs
    let toggle_sleep_id = MenuId::new("toggle_sleep");
    let toggle_autostart_id = MenuId::new("toggle_autostart");
    let screen_on_id = MenuId::new("screen_on");
    let screen_off_id = MenuId::new("screen_off");
    let quit_id = MenuId::new("quit");

    // Build menu items
    let toggle_sleep_text = if state.sleep_disabled {
        "Enable Sleep"
    } else {
        "Disable Sleep"
    };
    let toggle_sleep_item =
        MenuItemBuilder::with_id(toggle_sleep_id.clone(), toggle_sleep_text).build(handle)?;

    // Configure autostart
    let autostart_manager = handle.autolaunch();
    let is_autostart = autostart_manager.is_enabled().unwrap_or(false);

    if is_autostart {
        // Update autostart path if already enabled
        if autostart_manager.disable().is_ok() {
            if let Err(e) = autostart_manager.enable() {
                log::error!("Failed to update autostart path: {}", e);
            }
        }
    }

    let autostart_text = if is_autostart {
        "\u{2713} Start at Login"
    } else {
        "Start at Login"
    };
    let toggle_autostart_item =
        MenuItemBuilder::with_id(toggle_autostart_id.clone(), autostart_text).build(handle)?;

    let screen_on_text = if state.screen_mode == ScreenMode::KeepScreenOn {
        "\u{2713} Keep Screen On"
    } else {
        "Keep Screen On"
    };
    let screen_on_item =
        MenuItemBuilder::with_id(screen_on_id.clone(), screen_on_text).build(handle)?;

    let screen_off_text = if state.screen_mode == ScreenMode::AllowScreenOff {
        "\u{2713} Allow Screen Off"
    } else {
        "Allow Screen Off"
    };
    let screen_off_item =
        MenuItemBuilder::with_id(screen_off_id.clone(), screen_off_text).build(handle)?;

    let quit_item = MenuItemBuilder::with_id(quit_id.clone(), "Quit").build(handle)?;

    // Build tray menu
    let tray_menu = MenuBuilder::new(handle)
        .item(&toggle_sleep_item)
        .separator()
        .item(&screen_on_item)
        .item(&screen_off_item)
        .separator()
        .item(&toggle_autostart_item)
        .separator()
        .item(&quit_item)
        .build()?;

    // Wrap menu items for event handler
    let toggle_sleep_item = Arc::new(toggle_sleep_item);
    let toggle_sleep_item_clone = toggle_sleep_item.clone();
    let screen_on_item = Arc::new(screen_on_item);
    let screen_on_item_clone = screen_on_item.clone();
    let screen_off_item = Arc::new(screen_off_item);
    let screen_off_item_clone = screen_off_item.clone();

    // Generate initial tooltip
    let current_mode = *screen_mode.lock().unwrap();
    let tooltip = TooltipText::for_state(state.sleep_disabled, current_mode);

    // Load icon
    let icon_data = icon::get_icon_rgba(state.sleep_disabled)?;
    let tray = TrayIconBuilder::new()
        .icon(Image::new(icon_data.as_slice(), 32, 32))
        .menu(&tray_menu)
        .tooltip(tooltip.as_str())
        .build(handle)?;

    // Start wake service if needed
    if state.sleep_disabled {
        log::info!("Starting wake service on startup");
        start_wake_service(is_awake.clone(), current_mode);
    }

    let tray_handle = tray.clone();

    // Register menu event handler
    tray.on_menu_event(move |app, event| {
        if *event.id() == toggle_sleep_id {
            handle_toggle_sleep(
                is_awake.clone(),
                screen_mode.clone(),
                &toggle_sleep_item_clone,
                &tray_handle,
            );
        } else if *event.id() == screen_on_id {
            handle_screen_mode_change(
                ScreenMode::KeepScreenOn,
                is_awake.clone(),
                screen_mode.clone(),
                &screen_on_item_clone,
                &screen_off_item_clone,
                &tray_handle,
            );
        } else if *event.id() == screen_off_id {
            handle_screen_mode_change(
                ScreenMode::AllowScreenOff,
                is_awake.clone(),
                screen_mode.clone(),
                &screen_on_item_clone,
                &screen_off_item_clone,
                &tray_handle,
            );
        } else if *event.id() == toggle_autostart_id {
            handle_toggle_autostart(app, &toggle_autostart_item);
        } else if *event.id() == quit_id {
            handle_quit(app, is_awake.clone());
        }
    });

    app.manage(tray);
    Ok(())
}

/// Handle toggle sleep menu event
///
/// ## Design Intent
/// Delegates to core logic: updates state, persists, updates UI, starts/stops service.
///
/// ## Side Effects
/// - Modifies is_awake flag
/// - Writes state to disk
/// - Updates menu item text
/// - Updates tray icon and tooltip
/// - Starts or stops wake service
fn handle_toggle_sleep(
    is_awake: Arc<AtomicBool>,
    screen_mode: Arc<Mutex<ScreenMode>>,
    toggle_item: &Arc<tauri::menu::MenuItem<tauri::Wry>>,
    tray: &tauri::tray::TrayIcon<tauri::Wry>,
) {
    let was_awake = is_awake.load(Ordering::SeqCst);
    let new_awake = !was_awake;
    is_awake.store(new_awake, Ordering::SeqCst);

    log::info!("Toggling wake state: {} -> {}", was_awake, new_awake);

    // Persist state
    let current_mode = *screen_mode.lock().unwrap();
    let new_state = AppState {
        sleep_disabled: new_awake,
        screen_mode: current_mode,
    };
    if let Err(e) = write_state(&new_state) {
        log::error!("Failed to persist state: {}", e);
    }

    // Update UI
    let menu_text = if new_awake {
        "Enable Sleep"
    } else {
        "Disable Sleep"
    };
    let _ = toggle_item.set_text(menu_text);

    if let Ok(icon_data) = icon::get_icon_rgba(new_awake) {
        let tooltip = TooltipText::for_state(new_awake, current_mode);
        let _ = tray.set_icon(Some(Image::new(icon_data.as_slice(), 32, 32)));
        let _ = tray.set_tooltip(Some(tooltip.as_str()));
    }

    // Start or stop service
    if new_awake {
        start_wake_service(is_awake.clone(), current_mode);
    }
}

/// Handle screen mode change menu event
///
/// ## Design Intent
/// Updates screen mode preference and restarts wake service if active.
///
/// ## Side Effects
/// - Modifies screen_mode
/// - Writes state to disk
/// - Updates menu item checkmarks
/// - Restarts wake service if active
/// - Updates tooltip
fn handle_screen_mode_change(
    new_mode: ScreenMode,
    is_awake: Arc<AtomicBool>,
    screen_mode: Arc<Mutex<ScreenMode>>,
    screen_on_item: &Arc<tauri::menu::MenuItem<tauri::Wry>>,
    screen_off_item: &Arc<tauri::menu::MenuItem<tauri::Wry>>,
    tray: &tauri::tray::TrayIcon<tauri::Wry>,
) {
    log::info!("Changing screen mode to: {:?}", new_mode);

    *screen_mode.lock().unwrap() = new_mode;

    // Persist state
    let awake = is_awake.load(Ordering::SeqCst);
    let new_state = AppState {
        sleep_disabled: awake,
        screen_mode: new_mode,
    };
    if let Err(e) = write_state(&new_state) {
        log::error!("Failed to persist state: {}", e);
    }

    // Update menu checkmarks
    let _ = screen_on_item.set_text(if new_mode == ScreenMode::KeepScreenOn {
        "\u{2713} Keep Screen On"
    } else {
        "Keep Screen On"
    });
    let _ = screen_off_item.set_text(if new_mode == ScreenMode::AllowScreenOff {
        "\u{2713} Allow Screen Off"
    } else {
        "Allow Screen Off"
    });

    // Restart service if currently awake
    if awake {
        log::info!("Restarting wake service with new screen mode");
        is_awake.store(false, Ordering::SeqCst);
        std::thread::sleep(Duration::from_millis(100));
        is_awake.store(true, Ordering::SeqCst);
        start_wake_service(is_awake.clone(), new_mode);

        let tooltip = TooltipText::for_state(true, new_mode);
        let _ = tray.set_tooltip(Some(tooltip.as_str()));
    }
}

/// Handle toggle autostart menu event
///
/// ## Design Intent
/// Toggles autostart preference via Tauri plugin.
///
/// ## Side Effects
/// - Enables or disables autostart
/// - Updates menu item text
fn handle_toggle_autostart(
    app: &tauri::AppHandle,
    toggle_item: &Arc<tauri::menu::MenuItem<tauri::Wry>>,
) {
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap_or(false);

    log::info!("Toggling autostart: {} -> {}", is_enabled, !is_enabled);

    if is_enabled {
        let _ = autostart_manager.disable();
        let _ = toggle_item.set_text("Start at Login");
    } else {
        let _ = autostart_manager.enable();
        let _ = toggle_item.set_text("✓ Start at Login");
    }
}

/// Handle quit menu event
///
/// ## Design Intent
/// Clean shutdown - stop wake service and exit.
///
/// ## Side Effects
/// - Stops wake service
/// - Exits application
fn handle_quit(app: &tauri::AppHandle, is_awake: Arc<AtomicBool>) {
    log::info!("Quit requested");
    is_awake.store(false, Ordering::SeqCst);
    app.exit(0);
}

/// Start wake service in background
///
/// ## Design Intent
/// Spawns asynchronous wake service task.
///
/// ## Side Effects
/// - Spawns Tokio task
/// - Starts F15 simulation
/// - Sets platform display flags
fn start_wake_service(is_awake: Arc<AtomicBool>, screen_mode: ScreenMode) {
    let display_controller = platform::get_display_controller();
    let service = WakeService::new(is_awake, display_controller);

    tokio::spawn(async move {
        if let Err(e) = service.run(screen_mode).await {
            log::error!("Wake service error: {}", e);
        }
    });
}
