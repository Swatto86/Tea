//! Tooltip text generation
//!
//! Pure functions for generating context-appropriate tooltip text.
//!
//! ## Design Intent
//! Centralizes tooltip logic to ensure consistency and testability.
//!
//! ## Why separate
//! Tooltip generation is pure business logic with clear inputs/outputs.
//! Separating it from UI code allows unit testing and reuse.

use super::screen_mode::ScreenMode;

/// Tooltip text for tray icon
///
/// Wrapper type to ensure type safety when passing tooltip strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooltipText(String);

impl TooltipText {
    /// Generate tooltip for current application state
    ///
    /// ## Arguments
    /// * `is_awake` - Whether system wake is currently active
    /// * `screen_mode` - Current screen mode preference
    ///
    /// ## Returns
    /// Human-readable tooltip text describing current state
    pub fn for_state(is_awake: bool, screen_mode: ScreenMode) -> Self {
        let text = if is_awake {
            match screen_mode {
                ScreenMode::KeepScreenOn => "Tea - Screen & System On",
                ScreenMode::AllowScreenOff => "Tea - System On, Screen Can Sleep",
            }
        } else {
            "Tea - Sleep prevention disabled"
        };
        TooltipText(text.to_string())
    }

    /// Get the string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TooltipText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_when_disabled() {
        let tooltip = TooltipText::for_state(false, ScreenMode::default());
        assert_eq!(tooltip.as_str(), "Tea - Sleep prevention disabled");
    }

    #[test]
    fn test_tooltip_when_awake_with_screen_on() {
        let tooltip = TooltipText::for_state(true, ScreenMode::KeepScreenOn);
        assert_eq!(tooltip.as_str(), "Tea - Screen & System On");
    }

    #[test]
    fn test_tooltip_when_awake_with_screen_off_allowed() {
        let tooltip = TooltipText::for_state(true, ScreenMode::AllowScreenOff);
        assert_eq!(tooltip.as_str(), "Tea - System On, Screen Can Sleep");
    }

    #[test]
    fn test_screen_mode_does_not_affect_disabled_tooltip() {
        let tooltip1 = TooltipText::for_state(false, ScreenMode::KeepScreenOn);
        let tooltip2 = TooltipText::for_state(false, ScreenMode::AllowScreenOff);
        assert_eq!(tooltip1, tooltip2);
    }
}
