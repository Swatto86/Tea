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

// Embed icon files directly into the executable
static ICON_ALLOW: &[u8] = include_bytes!("../icons/icon-allow-32x32.png");
static ICON_BLOCK: &[u8] = include_bytes!("../icons/icon-block-32x32.png");

// Function to convert embedded icon data to RGBA
fn get_icon(is_sleep_disabled: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let icon_data = if is_sleep_disabled {
        ICON_BLOCK
    } else {
        ICON_ALLOW
    };
    
    let img = image::load_from_memory(icon_data)?;
    let rgba = img.into_rgba8();
    Ok(rgba.into_raw())
}

async fn keep_awake(running: Arc<AtomicBool>) {
    println!("Starting keep_awake function");
    let settings = Settings::default();
    let mut enigo = match Enigo::new(&settings) {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("Failed to initialize Enigo: {}", e);
            return;
        }
    };
    
    while running.load(Ordering::SeqCst) {
        println!("Simulating F15 key press...");
        match enigo.key(Key::F15, Direction::Click) {
            Ok(_) => println!("Key press successful - F15 key pressed"),
            Err(e) => eprintln!("Key press failed: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
    println!("Exiting keep_awake function");
}

#[tokio::main]
async fn main() {
    let sleep_disabled = Arc::new(AtomicBool::new(false));
    let sleep_disabled_clone = sleep_disabled.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .setup(move |app| {
            let handle = app.handle();
            let sleep_disabled = sleep_disabled_clone;
            
            let toggle_sleep_id = MenuId::new("toggle_sleep");
            let toggle_autostart_id = MenuId::new("toggle_autostart");
            let quit_id = MenuId::new("quit");
            
            let toggle_sleep_item = MenuItemBuilder::with_id(toggle_sleep_id.clone(), "Disable Sleep")
                .build(handle)?;
            
            // Get current autostart state
            let autostart_manager = handle.autolaunch();
            let is_autostart = autostart_manager.is_enabled().unwrap_or(false);
            
            let toggle_autostart_item = MenuItemBuilder::with_id(toggle_autostart_id.clone(), 
                if is_autostart { "\u{2713} Start at Login" } else { "Start at Login" })
                .build(handle)?;
            
            let quit_item = MenuItemBuilder::with_id(quit_id.clone(), "Quit")
                .build(handle)?;
            
            let tray_menu = MenuBuilder::new(handle)
                .item(&toggle_sleep_item)
                .item(&toggle_autostart_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let toggle_sleep_item = Arc::new(toggle_sleep_item);
            let toggle_sleep_item_clone = toggle_sleep_item.clone();

            let icon_data = get_icon(false).expect("Failed to get icon data");
            let tray = TrayIconBuilder::new()
                .icon(Image::new(icon_data.as_slice(), 32, 32))
                .menu(&tray_menu)
                .tooltip("Awake - Sleep prevention disabled")
                .build(handle)?;

            let tray_handle = tray.clone();
            
            tray.on_menu_event(move |app, event| {
                if *event.id() == toggle_sleep_id {
                    let is_disabled = sleep_disabled.load(Ordering::SeqCst);
                    if !is_disabled {
                        sleep_disabled.store(true, Ordering::SeqCst);
                        let _ = toggle_sleep_item_clone.set_text("Enable Sleep");
                        if let Ok(icon_data) = get_icon(true) {
                            let _ = tray_handle.set_icon(Some(Image::new(
                                icon_data.as_slice(),
                                32,
                                32
                            )));
                            let _ = tray_handle.set_tooltip(Some("Awake - Sleep prevention enabled"));
                        }
                        // Start the keep-awake task
                        let running = sleep_disabled.clone();
                        tokio::spawn(async move {
                            keep_awake(running).await;
                        });
                    } else {
                        sleep_disabled.store(false, Ordering::SeqCst);
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
