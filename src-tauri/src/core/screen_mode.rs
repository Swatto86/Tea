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

use serde::{Deserialize, Serialize};

/// User preference for screen behavior during wake periods
///
/// This setting controls whether the display should remain active when
/// the system is being kept awake.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenMode {
    /// Prevent display from sleeping or dimming
    ///
    /// On Windows: Sets ES_DISPLAY_REQUIRED flag
    /// On other platforms: No additional effect (relies on F15 simulation)
    KeepScreenOn,

    /// Allow display to sleep normally
    ///
    /// System remains awake via F15 simulation, but display follows
    /// normal power management settings.
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
    /// Used by platform adapters to determine whether to apply
    /// display-specific power flags.
    pub fn should_keep_display_on(self) -> bool {
        matches!(self, ScreenMode::KeepScreenOn)
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
}
