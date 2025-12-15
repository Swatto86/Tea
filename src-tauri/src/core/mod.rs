//! Core business logic module
//!
//! Contains pure, platform-agnostic logic with no I/O or external dependencies.
//! All functions here are deterministic and easily testable.

pub mod screen_mode;
pub mod tooltip;

pub use screen_mode::ScreenMode;
pub use tooltip::TooltipText;
