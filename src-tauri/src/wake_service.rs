//! System wake service
//!
//! Handles the background task that keeps the system awake via input simulation.
//!
//! ## Design Intent
//! Encapsulates the wake logic in a clean, testable service. Separates concerns:
//! - Input simulation (F15 key press - platform/mode dependent)
//! - Display control (platform-specific)
//! - Task lifecycle (start/stop)
//!
//! ## Why F15?
//! F15 was chosen because it is non-standard on most keyboards and therefore
//! unlikely to conflict with application shortcuts or user workflows. Most
//! applications don't bind actions to F15, making it safe to simulate without
//! interrupting user work.
//!
//! ## Side Effects
//! - On Windows with AllowScreenOff mode: Uses ES_CONTINUOUS API only (no F15)
//! - On Windows with KeepScreenOn mode: Uses ES_DISPLAY_REQUIRED + F15 for redundancy
//! - On non-Windows platforms: Simulates F15 key press every 60 seconds
//! - May set platform display power flags
//!
//! ## Failure Modes
//! - Input simulation initialization fails: Returns InputSimulation error (non-Windows or Windows KeepScreenOn)
//! - Key press fails: Logs error but continues running (transient failure)

use crate::core::ScreenMode;
use crate::error::{AppError, Result};
use crate::platform::DisplayControl;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Service that keeps system awake via periodic input simulation
///
/// ## Design Intent
/// Provides controlled lifecycle for wake functionality. Uses F15 key simulation
/// for maximum compatibility and adds platform-specific display control.
pub struct WakeService {
    /// Flag controlling whether wake loop continues
    running: Arc<AtomicBool>,
    /// Platform-specific display controller
    display_controller: Box<dyn DisplayControl + Send>,
}

impl WakeService {
    /// Create a new wake service
    ///
    /// ## Arguments
    /// * `running` - Shared flag to control service lifecycle
    /// * `display_controller` - Platform-specific display control implementation
    pub fn new(
        running: Arc<AtomicBool>,
        display_controller: Box<dyn DisplayControl + Send>,
    ) -> Self {
        Self {
            running,
            display_controller,
        }
    }

    /// Start keeping system awake
    ///
    /// ## Arguments
    /// * `screen_mode` - How to handle display power management
    ///
    /// ## Design Intent
    /// Main wake loop. Runs until `running` flag is set to false.
    /// On Windows with AllowScreenOff, uses ES_CONTINUOUS API alone (no F15) to allow screen sleep.
    /// On Windows with KeepScreenOn or non-Windows platforms, uses F15 simulation.
    ///
    /// ## Side Effects
    /// - On Windows AllowScreenOff: No F15 presses, screen can sleep normally
    /// - On Windows KeepScreenOn: Presses F15 every 60 seconds + ES_DISPLAY_REQUIRED
    /// - On non-Windows: Presses F15 every 60 seconds
    /// - Sets platform display flags based on screen_mode
    /// - Restores normal display mode on exit
    ///
    /// ## Failure Modes
    /// - Input initialization fails: Returns InputSimulation error (when F15 needed)
    /// - Individual key press fails: Logs error, continues running
    ///
    /// ## Returns
    /// Ok(()) when stopped normally, AppError::InputSimulation if initialization fails
    pub async fn run(self, screen_mode: ScreenMode) -> Result<()> {
        log::info!(
            "Starting wake service with screen mode: {:?}",
            screen_mode
        );

        // Apply platform display settings
        self.display_controller.set_display_mode(screen_mode);

        // Determine if F15 simulation is needed
        // On Windows with AllowScreenOff, ES_CONTINUOUS is sufficient - no F15 needed
        // This allows the screen to sleep while keeping system awake
        #[cfg(windows)]
        let use_f15 = screen_mode.should_keep_display_on();
        #[cfg(not(windows))]
        let use_f15 = true;

        log::info!(
            "Wake strategy: F15 simulation={}, platform API=active",
            use_f15
        );

        // Initialize input simulator only if needed
        let mut enigo = if use_f15 {
            let settings = Settings::default();
            Some(
                Enigo::new(&settings).map_err(|e| AppError::InputSimulation {
                    message: "Failed to initialize input simulator".to_string(),
                    cause: e.to_string(),
                    recovery_hint:
                        "Ensure the application has necessary permissions for input simulation.",
                })?,
            )
        } else {
            None
        };

        // Main wake loop
        while self.running.load(Ordering::SeqCst) {
            if let Some(ref mut enigo) = enigo {
                log::trace!("Simulating F15 key press (screen mode: {:?})", screen_mode);
                
                if let Err(e) = enigo.key(Key::F15, Direction::Click) {
                    log::error!("F15 key press failed (continuing): {}", e);
                } else {
                    log::trace!("F15 key press successful");
                }
            } else {
                log::trace!("Keeping system awake via platform API only (screen mode: {:?})", screen_mode);
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }

        // Restore normal display behavior
        self.display_controller.restore_normal_mode();
        log::info!("Wake service stopped");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ScreenMode;

    struct MockDisplayControl {
        calls: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl MockDisplayControl {
        fn new() -> (Self, Arc<std::sync::Mutex<Vec<String>>>) {
            let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
            (
                Self {
                    calls: calls.clone(),
                },
                calls,
            )
        }
    }

    impl DisplayControl for MockDisplayControl {
        fn set_display_mode(&self, screen_mode: ScreenMode) {
            self.calls
                .lock()
                .unwrap()
                .push(format!("set_display_mode({:?})", screen_mode));
        }

        fn restore_normal_mode(&self) {
            self.calls.lock().unwrap().push("restore_normal_mode".to_string());
        }
    }

    #[tokio::test]
    #[ignore] // Requires input simulation which may fail in CI/test environment
    async fn test_wake_service_lifecycle() {
        let running = Arc::new(AtomicBool::new(true));
        let (mock_display, calls) = MockDisplayControl::new();
        let service = WakeService::new(running.clone(), Box::new(mock_display));

        // Start service in background
        let running_clone = running.clone();
        let handle = tokio::spawn(async move {
            service.run(ScreenMode::KeepScreenOn).await
        });

        // Let it initialize
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Stop service
        running_clone.store(false, Ordering::SeqCst);

        // Wait for completion - must complete for restore to be called
        let result = tokio::time::timeout(Duration::from_secs(3), handle).await;
        assert!(result.is_ok(), "Service should complete within timeout");

        // Give time for cleanup to complete
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify display control was called
        let call_log = calls.lock().unwrap();
        assert!(
            call_log.contains(&"set_display_mode(KeepScreenOn)".to_string()),
            "set_display_mode should be called"
        );
        assert!(
            call_log.contains(&"restore_normal_mode".to_string()),
            "restore_normal_mode should be called. Calls: {:?}",
            *call_log
        );
    }
}
