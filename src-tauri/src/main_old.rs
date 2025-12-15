// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, MenuId},
    tray::TrayIconBuilder,
    Manager,
    image::Image,
};
use tauri_plugin_autostart::{ManagerExt, MacosLauncher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use enigo::{Enigo, Key, Direction, Settings, Keyboard};
use std::fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::env;

#[cfg(windows)]
use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED};

// Embed icon files directly into the executable
static ICON_ALLOW: &[u8] = include_bytes!("../icons/icon-allow-32x32.png");
static ICON_BLOCK: &[u8] = include_bytes!("../icons/icon-block-32x32.png");

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum ScreenMode {
    KeepScreenOn,
    AllowScreenOff,
}

impl Default for ScreenMode {
    fn default() -> Self {
        ScreenMode::AllowScreenOff
    }
}

#[derive(Serialize, Deserialize, Default)]
struct AppState {
    sleep_disabled: bool,
    screen_mode: ScreenMode,
}

fn get_state_file_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let xdg_config = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        let mut path = PathBuf::from(xdg_config);
        path.push("awake");
        fs::create_dir_all(&path).unwrap_or_default();
        path.push("state.json");
        path
    }
    #[cfg(not(target_os = "linux"))]
    {
        let mut path = env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        path.push("config");
        fs::create_dir_all(&path).unwrap_or_default();
        path.push("state.json");
        path
    }
}

fn save_state(sleep_disabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = load_state();
    state.sleep_disabled = sleep_disabled;
    let path = get_state_file_path();
    let json = serde_json::to_string_pretty(&state)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_state() -> AppState {
    let path = get_state_file_path();
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppState::default(),
    }
}

// Function to convert embedded icon data to RGBA
fn get_icon(is_sleep_disabled: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let icon_data = if is_sleep_disabled { ICON_BLOCK } else { ICON_ALLOW };
    let img = image::load_from_memory(icon_data)?;
    let rgba = img.into_rgba8();
    Ok(rgba.into_raw())
}

// Platform-specific function to control screen using Windows API
#[cfg(windows)]
fn set_display_required(keep_screen_on: bool) {
    unsafe {
        if keep_screen_on {
            SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED);
        } else {
            SetThreadExecutionState(ES_CONTINUOUS);
        }
    }
}

#[cfg(not(windows))]
fn set_display_required(_keep_screen_on: bool) {}

async fn keep_awake(running: Arc<AtomicBool>, screen_mode: ScreenMode) {
    println!("Starting keep_awake function with screen mode: {:?}", screen_mode);
    
    let settings = Settings::default();
    let mut enigo = match Enigo::new(&settings) {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("Failed to initialize Enigo: {}", e);
            return;
        }
    };
    
    // On Windows, set display state based on screen mode
    #[cfg(windows)]
    set_display_required(screen_mode == ScreenMode::KeepScreenOn);
    
    // Use F15 key simulation to keep system awake in all cases
    while running.load(Ordering::SeqCst) {
        println!("Simulating F15 key press... (screen mode: {:?})", screen_mode);
        match enigo.key(Key::F15, Direction::Click) {
            Ok(_) => println!("Key press successful - F15 key pressed"),
            Err(e) => eprintln!("Key press failed: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
    
    // Restore normal power settings on Windows
    #[cfg(windows)]
    set_display_required(false);
    
    println!("Exiting keep_awake function");
}

#[tokio::main]
async fn main() {
    let state = load_state();
    let sleep_disabled = Arc::new(AtomicBool::new(state.sleep_disabled));
    let sleep_disabled_clone = sleep_disabled.clone();
    let screen_mode = Arc::new(std::sync::Mutex::new(state.screen_mode));
    let screen_mode_clone = screen_mode.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .setup(move |app| {
            let handle = app.handle();
            let sleep_disabled = sleep_disabled_clone;
            let screen_mode = screen_mode_clone;
            
            let toggle_sleep_id = MenuId::new("toggle_sleep");
            let toggle_autostart_id = MenuId::new("toggle_autostart");
            let screen_on_id = MenuId::new("screen_on");
            let screen_off_id = MenuId::new("screen_off");
            let quit_id = MenuId::new("quit");
            
            let toggle_sleep_item = MenuItemBuilder::with_id(toggle_sleep_id.clone(), 
                if state.sleep_disabled { "Enable Sleep" } else { "Disable Sleep" })
                .build(handle)?;
            
            // Get current autostart state and update path if enabled
            let autostart_manager = handle.autolaunch();
            let is_autostart = autostart_manager.is_enabled().unwrap_or(false);
            
            // If autostart is enabled, update the path to current executable
            if is_autostart {
                // Disable and re-enable to update the path
                if let Ok(()) = autostart_manager.disable() {
                    if let Err(e) = autostart_manager.enable() {
                        eprintln!("Failed to update autostart path: {}", e);
                    }
                }
            }
            
            let toggle_autostart_item = MenuItemBuilder::with_id(toggle_autostart_id.clone(), 
                if is_autostart { "\u{2713} Start at Login" } else { "Start at Login" })
                .build(handle)?;
            
            let screen_on_item = MenuItemBuilder::with_id(screen_on_id.clone(), 
                if state.screen_mode == ScreenMode::KeepScreenOn { "\u{2713} Keep Screen On" } else { "Keep Screen On" })
                .build(handle)?;
            
            let screen_off_item = MenuItemBuilder::with_id(screen_off_id.clone(), 
                if state.screen_mode == ScreenMode::AllowScreenOff { "\u{2713} Allow Screen Off" } else { "Allow Screen Off" })
                .build(handle)?;
            
            let quit_item = MenuItemBuilder::with_id(quit_id.clone(), "Quit")
                .build(handle)?;
            
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

            let toggle_sleep_item = Arc::new(toggle_sleep_item);
            let toggle_sleep_item_clone = toggle_sleep_item.clone();
            let screen_on_item = Arc::new(screen_on_item);
            let screen_on_item_clone = screen_on_item.clone();
            let screen_off_item = Arc::new(screen_off_item);
            let screen_off_item_clone = screen_off_item.clone();

            let tooltip_text = if state.sleep_disabled {
                match state.screen_mode {
                    ScreenMode::KeepScreenOn => "Awake - Screen & System On",
                    ScreenMode::AllowScreenOff => "Awake - System On, Screen Can Sleep",
                }
            } else {
                "Awake - Sleep prevention disabled"
            };

            let icon_data = get_icon(state.sleep_disabled).expect("Failed to get icon data");
            let tray = TrayIconBuilder::new()
                .icon(Image::new(icon_data.as_slice(), 32, 32))
                .menu(&tray_menu)
                .tooltip(tooltip_text)
                .build(handle)?;

            // If sleep was disabled in previous session, start the keep-awake task
            if state.sleep_disabled {
                let running = sleep_disabled.clone();
                let mode = *screen_mode.lock().unwrap();
                tokio::spawn(async move {
                    keep_awake(running, mode).await;
                });
            }

            let tray_handle = tray.clone();
            
            tray.on_menu_event(move |app, event| {
                if *event.id() == toggle_sleep_id {
                    let is_disabled = sleep_disabled.load(Ordering::SeqCst);
                    if !is_disabled {
                        sleep_disabled.store(true, Ordering::SeqCst);
                        let mode = *screen_mode.lock().unwrap();
                        let mut state = load_state();
                        state.sleep_disabled = true;
                        let _ = save_state(state.sleep_disabled);
                        let _ = toggle_sleep_item_clone.set_text("Enable Sleep");
                        if let Ok(icon_data) = get_icon(true) {
                            let tooltip = match mode {
                                ScreenMode::KeepScreenOn => "Awake - Screen & System On",
                                ScreenMode::AllowScreenOff => "Awake - System On, Screen Can Sleep",
                            };
                            let _ = tray_handle.set_icon(Some(Image::new(
                                icon_data.as_slice(),
                                32,
                                32
                            )));
                            let _ = tray_handle.set_tooltip(Some(tooltip));
                        }
                        let running = sleep_disabled.clone();
                        tokio::spawn(async move {
                            keep_awake(running, mode).await;
                        });
                    } else {
                        sleep_disabled.store(false, Ordering::SeqCst);
                        let mut state = load_state();
                        state.sleep_disabled = false;
                        let _ = save_state(state.sleep_disabled);
                        let _ = toggle_sleep_item_clone.set_text("Disable Sleep");
                        if let Ok(icon_data) = get_icon(false) {
                            let _ = tray_handle.set_icon(Some(Image::new(
                                icon_data.as_slice(),
                                32,
                                32
                            )));
                            let _ = tray_handle.set_tooltip(Some("Awake - Sleep prevention disabled"));
                        }
                    }
                } else if *event.id() == screen_on_id {
                    *screen_mode.lock().unwrap() = ScreenMode::KeepScreenOn;
                    let mut state = load_state();
                    state.screen_mode = ScreenMode::KeepScreenOn;
                    let _ = (|| -> Result<(), Box<dyn std::error::Error>> {
                        let json = serde_json::to_string_pretty(&state)?;
                        fs::write(get_state_file_path(), json)?;
                        Ok(())
                    })();
                    
                    let _ = screen_on_item_clone.set_text("\u{2713} Keep Screen On");
                    let _ = screen_off_item_clone.set_text("Allow Screen Off");
                    
                    // If currently awake, restart the task with new mode
                    if sleep_disabled.load(Ordering::SeqCst) {
                        sleep_disabled.store(false, Ordering::SeqCst);
                        std::thread::sleep(Duration::from_millis(100));
                        sleep_disabled.store(true, Ordering::SeqCst);
                        let running = sleep_disabled.clone();
                        let mode = ScreenMode::KeepScreenOn;
                        tokio::spawn(async move {
                            keep_awake(running, mode).await;
                        });
                        let _ = tray_handle.set_tooltip(Some("Awake - Screen & System On"));
                    }
                } else if *event.id() == screen_off_id {
                    *screen_mode.lock().unwrap() = ScreenMode::AllowScreenOff;
                    let mut state = load_state();
                    state.screen_mode = ScreenMode::AllowScreenOff;
                    let _ = (|| -> Result<(), Box<dyn std::error::Error>> {
                        let json = serde_json::to_string_pretty(&state)?;
                        fs::write(get_state_file_path(), json)?;
                        Ok(())
                    })();
                    
                    let _ = screen_on_item_clone.set_text("Keep Screen On");
                    let _ = screen_off_item_clone.set_text("\u{2713} Allow Screen Off");
                    
                    // If currently awake, restart the task with new mode
                    if sleep_disabled.load(Ordering::SeqCst) {
                        sleep_disabled.store(false, Ordering::SeqCst);
                        std::thread::sleep(Duration::from_millis(100));
                        sleep_disabled.store(true, Ordering::SeqCst);
                        let running = sleep_disabled.clone();
                        let mode = ScreenMode::AllowScreenOff;
                        tokio::spawn(async move {
                            keep_awake(running, mode).await;
                        });
                        let _ = tray_handle.set_tooltip(Some("Awake - System On, Screen Can Sleep"));
                    }
                } else if *event.id() == toggle_autostart_id {
                    let autostart_manager = app.autolaunch();
                    let is_enabled = autostart_manager.is_enabled().unwrap_or(false);
                    
                    if is_enabled {
                        let _ = autostart_manager.disable();
                        let _ = toggle_autostart_item.set_text("Start at Login");
                    } else {
                        let _ = autostart_manager.enable();
                        let _ = toggle_autostart_item.set_text("✓ Start at Login");
                    }
                } else if *event.id() == quit_id {
                    sleep_disabled.store(false, Ordering::SeqCst);
                    app.exit(0);
                }
            });

            app.manage(tray);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
