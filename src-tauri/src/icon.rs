//! Icon utilities
//!
//! Loads and processes embedded icon data for tray icon display.
//!
//! ## Design Intent
//! Isolates icon processing logic with explicit error handling.
//!
//! ## Side Effects
//! None - pure image processing of embedded data.
//!
//! ## Failure Modes
//! - Corrupted icon data: Returns IconProcessing error

use crate::error::{AppError, Result};

/// Embedded icon for "sleep allowed" state (gray)
static ICON_ALLOW: &[u8] = include_bytes!("../icons/icon-allow-32x32.png");

/// Embedded icon for "sleep blocked" state (green)
static ICON_BLOCK: &[u8] = include_bytes!("../icons/icon-block-32x32.png");

/// Convert embedded icon data to RGBA format
///
/// ## Design Intent
/// Prepares icon data for display by Tauri tray icon API.
///
/// ## Arguments
/// * `is_awake` - Whether to return the "awake" or "sleep" icon
///
/// ## Returns
/// RGBA pixel data on success, AppError::IconProcessing on failure
///
/// ## Failure Modes
/// - Corrupted embedded data: Returns IconProcessing error
pub fn get_icon_rgba(is_awake: bool) -> Result<Vec<u8>> {
    let icon_data = if is_awake { ICON_BLOCK } else { ICON_ALLOW };

    let img = image::load_from_memory(icon_data).map_err(|e| AppError::IconProcessing {
        message: format!(
            "Failed to load {} icon from embedded data",
            if is_awake { "awake" } else { "sleep" }
        ),
        cause: e.to_string(),
        recovery_hint: "This is a bug. Icon data may be corrupted.",
    })?;

    let rgba = img.into_rgba8();
    Ok(rgba.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_icon_for_awake_state() {
        let result = get_icon_rgba(true);
        assert!(result.is_ok());
        let data = result.unwrap();
        // 32x32 RGBA = 4096 bytes
        assert_eq!(data.len(), 32 * 32 * 4);
    }

    #[test]
    fn test_get_icon_for_sleep_state() {
        let result = get_icon_rgba(false);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 32 * 32 * 4);
    }

    #[test]
    fn test_icons_are_different() {
        let awake_icon = get_icon_rgba(true).unwrap();
        let sleep_icon = get_icon_rgba(false).unwrap();
        assert_ne!(awake_icon, sleep_icon);
    }
}
