//! Application state persistence
//!
//! Handles reading and writing application state to disk.
//!
//! ## Design Intent
//! Isolates all filesystem I/O for state management. Provides clear error
//! handling and recovery paths.
//!
//! ## Side Effects
//! - Reads from config directory
//! - Writes to config directory
//! - Creates directories as needed
//!
//! ## Failure Modes
//! - Disk full: Returns StateIo error with recovery hint to free space
//! - Permission denied: Returns StateIo error with recovery hint to check permissions
//! - Corrupted state: Returns default state (defensive design)

use crate::core::ScreenMode;
use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application state persisted between sessions
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    /// Whether system wake is currently active
    pub sleep_disabled: bool,
    /// User's screen mode preference
    pub screen_mode: ScreenMode,
}

/// Get the path to the state file
///
/// ## Design Intent
/// Centralizes path logic to ensure consistency across load/save operations.
///
/// ## Platform Behavior
/// - Windows: Uses %LOCALAPPDATA%\tea\state.json
/// - Linux: Uses XDG_CONFIG_HOME or ~/.config/tea/state.json
/// - macOS: Uses ~/Library/Application Support/tea/state.json
///
/// ## Side Effects
/// Creates parent directories if they don't exist.
///
/// ## Returns
/// Result with path to state file. Parent directories are guaranteed to exist
/// if function succeeds. Returns StateIo error if directory creation fails.
fn get_state_file_path() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(local_app_data);
        path.push("tea");
        fs::create_dir_all(&path).map_err(|e| AppError::StateIo {
            message: format!("Failed to create config directory at {}", path.display()),
            cause: e.to_string(),
            recovery_hint: "Ensure you have write permissions to the AppData directory.",
        })?;
        path.push("state.json");
        Ok(path)
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let xdg_config = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        let mut path = PathBuf::from(xdg_config);
        path.push("tea");
        fs::create_dir_all(&path).map_err(|e| AppError::StateIo {
            message: format!("Failed to create config directory at {}", path.display()),
            cause: e.to_string(),
            recovery_hint: "Ensure you have write permissions to the config directory.",
        })?;
        path.push("state.json");
        Ok(path)
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push("Library");
        path.push("Application Support");
        path.push("tea");
        fs::create_dir_all(&path).map_err(|e| AppError::StateIo {
            message: format!("Failed to create config directory at {}", path.display()),
            cause: e.to_string(),
            recovery_hint: "Ensure you have write permissions to the Application Support directory.",
        })?;
        path.push("state.json");
        Ok(path)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        // Fallback for other platforms
        let mut path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        path.push("config");
        fs::create_dir_all(&path).map_err(|e| AppError::StateIo {
            message: format!("Failed to create config directory at {}", path.display()),
            cause: e.to_string(),
            recovery_hint: "Ensure you have write permissions to the application directory.",
        })?;
        path.push("state.json");
        Ok(path)
    }
}

/// Write application state to disk
///
/// ## Design Intent
/// Persists user preferences so they survive application restarts.
///
/// ## Arguments
/// * `state` - The state to persist
///
/// ## Side Effects
/// - Writes to config directory
/// - Overwrites existing state file
///
/// ## Returns
/// Ok(()) on success, AppError::StateIo or AppError::StateSerialization on failure
pub fn write_state(state: &AppState) -> Result<()> {
    let path = get_state_file_path()?;
    
    let json = serde_json::to_string_pretty(state).map_err(|e| AppError::StateSerialization {
        message: "Failed to serialize application state".to_string(),
        cause: e.to_string(),
        recovery_hint: "This is a bug. Please report it with your state configuration.",
    })?;

    fs::write(&path, json).map_err(|e| AppError::StateIo {
        message: format!("Failed to write state to {}", path.display()),
        cause: e.to_string(),
        recovery_hint: "Ensure you have write permissions and sufficient disk space.",
    })?;

    Ok(())
}

/// Read application state from disk
///
/// ## Design Intent
/// Restores user preferences from previous session.
///
/// ## Side Effects
/// Reads from config directory
///
/// ## Returns
/// Loaded state on success, or default state if file doesn't exist or is corrupted.
/// Never fails - returns default state as fallback.
pub fn read_state() -> AppState {
    let path = match get_state_file_path() {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to get state file path, using defaults: {}", e);
            return AppState::default();
        }
    };
    
    match fs::read_to_string(&path) {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(state) => state,
                Err(e) => {
                    log::warn!(
                        "State file corrupted ({}), using defaults: {}",
                        path.display(),
                        e
                    );
                    AppState::default()
                }
            }
        }
        Err(e) => {
            // File not existing is normal on first run
            if e.kind() != std::io::ErrorKind::NotFound {
                log::warn!("Failed to read state file, using defaults: {}", e);
            }
            AppState::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_values() {
        let state = AppState::default();
        assert!(!state.sleep_disabled);
        assert_eq!(state.screen_mode, ScreenMode::AllowScreenOff);
    }

    #[test]
    fn test_state_serialization() {
        let state = AppState {
            sleep_disabled: true,
            screen_mode: ScreenMode::KeepScreenOn,
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: AppState = serde_json::from_str(&json).unwrap();

        assert_eq!(state, deserialized);
    }
}
