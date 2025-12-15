//! Platform abstraction for system power management
//!
//! Defines traits and implementations for platform-specific power control.
//!
//! ## Design Intent
//! Isolates all Windows API calls behind a clean trait boundary. This allows:
//! - Easy testing with mock implementations
//! - Platform-agnostic core logic
//! - Clear documentation of platform capabilities
//!
//! ## Side Effects
//! Implementations may modify system power settings via platform APIs.

use crate::core::ScreenMode;

/// Platform-specific display power control
///
/// ## Design Intent
/// This trait abstracts display power management from core wake logic.
/// Implementations use platform-specific APIs (Windows SetThreadExecutionState,
/// etc.) without leaking those details to business logic.
pub trait DisplayControl {
    /// Apply display power requirements
    ///
    /// ## Arguments
    /// * `screen_mode` - Desired screen behavior
    ///
    /// ## Side Effects
    /// May set platform power flags that affect display sleep behavior.
    fn set_display_mode(&self, screen_mode: ScreenMode);

    /// Restore normal display power behavior
    ///
    /// ## Side Effects
    /// Clears any display-related power flags set by this controller.
    fn restore_normal_mode(&self);
}

/// Windows-specific display control using SetThreadExecutionState
///
/// ## Platform
/// Windows only. Uses Win32 Power Management API.
///
/// ## Behavior
/// - KeepScreenOn: Sets ES_CONTINUOUS | ES_DISPLAY_REQUIRED
/// - AllowScreenOff: Sets ES_CONTINUOUS only
///
/// ## Safety
/// Uses unsafe Windows API calls. Platform guarantees these are safe when
/// called from application context.
#[cfg(windows)]
pub struct WindowsDisplayControl;

#[cfg(windows)]
impl DisplayControl for WindowsDisplayControl {
    fn set_display_mode(&self, screen_mode: ScreenMode) {
        use windows::Win32::System::Power::{
            SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED,
        };

        unsafe {
            if screen_mode.should_keep_display_on() {
                log::debug!("Setting Windows display mode: keep screen on");
                SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED);
            } else {
                log::debug!("Setting Windows display mode: allow screen off");
                SetThreadExecutionState(ES_CONTINUOUS);
            }
        }
    }

    fn restore_normal_mode(&self) {
        use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS};

        unsafe {
            log::debug!("Restoring Windows normal power mode");
            SetThreadExecutionState(ES_CONTINUOUS);
        }
    }
}

/// No-op display control for platforms without specific support
///
/// ## Platform
/// Non-Windows platforms
///
/// ## Behavior
/// Does nothing. Screen behavior is controlled by F15 simulation only.
#[cfg(not(windows))]
pub struct NoOpDisplayControl;

#[cfg(not(windows))]
impl DisplayControl for NoOpDisplayControl {
    fn set_display_mode(&self, _screen_mode: ScreenMode) {
        // No platform-specific display control available
    }

    fn restore_normal_mode(&self) {
        // No platform-specific display control to restore
    }
}

/// Get the platform-appropriate display controller
///
/// ## Design Intent
/// Factory function that returns the correct implementation for current platform.
/// Allows platform-agnostic code to obtain a display controller without
/// conditional compilation at call sites.
pub fn get_display_controller() -> Box<dyn DisplayControl + Send> {
    #[cfg(windows)]
    {
        Box::new(WindowsDisplayControl)
    }

    #[cfg(not(windows))]
    {
        Box::new(NoOpDisplayControl)
    }
}
