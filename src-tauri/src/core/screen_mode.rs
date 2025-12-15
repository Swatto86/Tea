//! Screen mode configuration
//!
//! Defines the user's preference for screen behavior during wake periods.
//!
//! ## Design Intent
//! This type exists to cleanly separate screen control from system wake logic.
//! It allows platform-specific implementations to interpret the mode appropriately.
//!
//! ## Why separate from other types
//! Screen control is independent of system wake state and persistence logic.
//! Keeping it separate allows easy extension (e.g., adding timed modes).
//!
//! ## Platform Support
//! AllowScreenOff is only supported on Windows where ES_SYSTEM_REQUIRED can keep
//! the system awake without F15 simulation. On other platforms, F15 simulation
//! prevents both system and display sleep, making AllowScreenOff impossible.

use serde::{Deserialize, Serialize};

/// User preference for screen behavior during wake periods
///
/// This setting controls whether the display should remain active when
/// the system is being kept awake.
///
/// ## Platform Constraints
/// AllowScreenOff requires platform-specific APIs (Windows SetThreadExecutionState)
/// to keep system awake without input simulation. Not all modes are available
/// on all platforms.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenMode {
    /// Prevent display from sleeping or dimming
    ///
    /// Supported on all platforms.
    ///
    /// On Windows: Sets ES_DISPLAY_REQUIRED flag + F15 simulation
    /// On other platforms: F15 simulation (prevents both system and display sleep)
    KeepScreenOn,

    /// Allow display to sleep normally while keeping system awake
    ///
    /// **Windows only** - Not supported on macOS/Linux.
    ///
    /// On Windows: Uses ES_SYSTEM_REQUIRED without F15 (allows display sleep)
    /// On other platforms: Not available (would require F15 which prevents display sleep)
    AllowScreenOff,
}

impl Default for ScreenMode {
    fn default() -> Self {
        ScreenMode::AllowScreenOff
    }
}

impl ScreenMode {
    /// Returns true if this mode requires display to stay active
    ///
    /// ## Design Intent
    /// Used by platform adapters to determine whether to apply
    /// display-specific power flags.
    pub fn should_keep_display_on(self) -> bool {
        matches!(self, ScreenMode::KeepScreenOn)
    }

    /// Returns true if this mode is supported on the current platform
    ///
    /// ## Design Intent
    /// Core business logic for platform capability detection.
    /// UI layer uses this to determine which menu items to show.
    ///
    /// ## Platform Behavior
    /// - KeepScreenOn: Supported on all platforms
    /// - AllowScreenOff: Windows only (requires ES_SYSTEM_REQUIRED without F15)
    ///
    /// ## Why this exists
    /// On non-Windows platforms, preventing system sleep requires F15 simulation,
    /// which also prevents display sleep. Therefore AllowScreenOff cannot work
    /// as intended on those platforms.
    pub fn is_supported(self) -> bool {
        match self {
            ScreenMode::KeepScreenOn => true,
            ScreenMode::AllowScreenOff => cfg!(windows),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_allows_screen_off() {
        assert_eq!(ScreenMode::default(), ScreenMode::AllowScreenOff);
    }

    #[test]
    fn test_keep_screen_on_requires_display() {
        assert!(ScreenMode::KeepScreenOn.should_keep_display_on());
    }

    #[test]
    fn test_allow_screen_off_does_not_require_display() {
        assert!(!ScreenMode::AllowScreenOff.should_keep_display_on());
    }

    #[test]
    fn test_screen_modes_are_distinct() {
        assert_ne!(ScreenMode::KeepScreenOn, ScreenMode::AllowScreenOff);
    }

    // Platform capability tests (Principle 12: Tests where logic exists)
    #[test]
    fn test_keep_screen_on_always_supported() {
        // KeepScreenOn must be supported on all platforms
        assert!(ScreenMode::KeepScreenOn.is_supported());
    }

    #[test]
    #[cfg(windows)]
    fn test_allow_screen_off_supported_on_windows() {
        // AllowScreenOff is supported on Windows (ES_SYSTEM_REQUIRED API available)
        assert!(ScreenMode::AllowScreenOff.is_supported());
    }

    #[test]
    #[cfg(not(windows))]
    fn test_allow_screen_off_not_supported_on_non_windows() {
        // AllowScreenOff is NOT supported on non-Windows (F15 prevents display sleep)
        assert!(!ScreenMode::AllowScreenOff.is_supported());
    }
}
