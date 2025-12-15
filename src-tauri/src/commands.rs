//! Tauri command layer
//!
//! Provides clean, testable commands for UI interactions.
//!
//! ## Design Intent
//! Separates business logic from UI event handling. All commands are:
//! - Single-purpose
//! - Return structured results
//! - Have no direct UI dependencies
//!
//! ## Architecture
//! Commands orchestrate core logic, persistence, and wake service.
//! UI handlers simply delegate to these commands.

use crate::core::ScreenMode;
use crate::persistence::{write_state, AppState};
use crate::platform;
use crate::wake_service::WakeService;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::State;

/// Shared application state managed by Tauri
pub struct AppStateManager {
    pub is_awake: Arc<AtomicBool>,
    pub screen_mode: Arc<Mutex<ScreenMode>>,
}

/// Internal business logic for toggling sleep state
///
/// ## Design Intent
/// Shared logic called by both Tauri commands (frontend) and menu handlers (tray).
/// Keeps business logic in one place.
///
/// ## Arguments
/// * `is_awake` - Shared atomic flag
/// * `screen_mode` - Shared mutex with screen mode
///
/// ## Returns
/// New awake state and screen mode, or error string
pub fn toggle_sleep_impl(
    is_awake: &Arc<AtomicBool>,
    screen_mode: &Arc<Mutex<ScreenMode>>,
) -> Result<(bool, ScreenMode), String> {
    let was_awake = is_awake.load(Ordering::SeqCst);
    let new_awake = !was_awake;
    is_awake.store(new_awake, Ordering::SeqCst);

    log::info!("Toggle sleep: {} -> {}", was_awake, new_awake);

    // Get current screen mode with proper poisoning handling
    let current_mode = *screen_mode
        .lock()
        .map_err(|e| format!("Mutex poisoned during toggle_sleep: {}", e))?;

    // Persist state
    let new_state = AppState {
        sleep_disabled: new_awake,
        screen_mode: current_mode,
    };
    write_state(&new_state).map_err(|e| format!("Failed to persist state: {}", e))?;

    // Start service if needed
    if new_awake {
        start_wake_service(is_awake.clone(), current_mode);
    }

    Ok((new_awake, current_mode))
}

/// Toggle system sleep prevention (Tauri command for frontend)
///
/// ## Design Intent
/// Frontend-facing API that delegates to shared business logic.
///
/// ## Arguments
/// * `state` - Managed application state
///
/// ## Returns
/// New awake state and screen mode, or error string
#[tauri::command]
pub fn toggle_sleep(state: State<AppStateManager>) -> Result<(bool, ScreenMode), String> {
    toggle_sleep_impl(&state.is_awake, &state.screen_mode)
}

/// Internal business logic for changing screen mode
///
/// ## Design Intent
/// Shared logic called by both Tauri commands (frontend) and menu handlers (tray).
/// Keeps business logic in one place.
///
/// ## Arguments
/// * `is_awake` - Shared atomic flag
/// * `screen_mode` - Shared mutex with screen mode
/// * `new_mode` - Desired screen mode
///
/// ## Returns
/// New screen mode, or error string
pub fn change_screen_mode_impl(
    is_awake: &Arc<AtomicBool>,
    screen_mode: &Arc<Mutex<ScreenMode>>,
    new_mode: ScreenMode,
) -> Result<ScreenMode, String> {
    log::info!("Change screen mode to {:?}", new_mode);

    // Update screen mode with proper poisoning handling
    {
        let mut mode = screen_mode
            .lock()
            .map_err(|e| format!("Mutex poisoned during change_screen_mode: {}", e))?;
        *mode = new_mode;
    }

    // Persist state
    let awake = is_awake.load(Ordering::SeqCst);
    let new_state = AppState {
        sleep_disabled: awake,
        screen_mode: new_mode,
    };
    write_state(&new_state).map_err(|e| format!("Failed to persist state: {}", e))?;

    // Restart service if currently awake
    if awake {
        log::info!("Restarting wake service with new screen mode");
        is_awake.store(false, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_millis(100));
        is_awake.store(true, Ordering::SeqCst);
        start_wake_service(is_awake.clone(), new_mode);
    }

    Ok(new_mode)
}

/// Change screen mode preference (Tauri command for frontend)
///
/// ## Design Intent
/// Frontend-facing API that delegates to shared business logic.
///
/// ## Arguments
/// * `state` - Managed application state
/// * `new_mode` - Desired screen mode
///
/// ## Returns
/// New screen mode, or error string
#[tauri::command]
pub fn change_screen_mode(
    state: State<AppStateManager>,
    new_mode: ScreenMode,
) -> Result<ScreenMode, String> {
    change_screen_mode_impl(&state.is_awake, &state.screen_mode, new_mode)
}

/// Get current application state
///
/// ## Design Intent
/// Provides UI with current state for rendering.
///
/// ## Returns
/// Current awake state and screen mode, or error string
#[tauri::command]
pub fn get_state(state: State<AppStateManager>) -> Result<(bool, ScreenMode), String> {
    let awake = state.is_awake.load(Ordering::SeqCst);
    let mode = *state
        .screen_mode
        .lock()
        .map_err(|e| format!("Mutex poisoned during get_state: {}", e))?;

    Ok((awake, mode))
}

/// Start wake service in background
///
/// ## Design Intent
/// Spawns asynchronous wake service task. Used by both business logic
/// and startup initialization.
///
/// ## Side Effects
/// - Spawns Tokio task
/// - Starts F15 simulation
/// - Sets platform display flags
pub fn start_wake_service(is_awake: Arc<AtomicBool>, screen_mode: ScreenMode) {
    let display_controller = platform::get_display_controller();
    let service = WakeService::new(is_awake, display_controller);

    tokio::spawn(async move {
        if let Err(e) = service.run(screen_mode).await {
            log::error!("Wake service error: {}", e);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_manager_creation() {
        let manager = AppStateManager {
            is_awake: Arc::new(AtomicBool::new(false)),
            screen_mode: Arc::new(Mutex::new(ScreenMode::default())),
        };

        assert!(!manager.is_awake.load(Ordering::SeqCst));
        assert_eq!(
            *manager.screen_mode.lock().unwrap(),
            ScreenMode::AllowScreenOff
        );
    }
}
