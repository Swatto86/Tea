//! Unit tests for Awake application
//! Tests state management, icon handling, and utility functions

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct AppState {
    sleep_disabled: bool,
}

// Mock implementations of the functions from main.rs for testing
fn get_test_state_file_path(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("awake_test_{}", test_name));
    fs::create_dir_all(&path).unwrap_or_default();
    path.push("state.json");
    path
}

fn save_test_state(
    sleep_disabled: bool,
    test_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { sleep_disabled };
    let path = get_test_state_file_path(test_name);
    let json = serde_json::to_string_pretty(&state)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_test_state(test_name: &str) -> AppState {
    let path = get_test_state_file_path(test_name);
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppState::default(),
    }
}

fn cleanup_test_state(test_name: &str) {
    let path = get_test_state_file_path(test_name);
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent);
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = AppState::default();
        assert_eq!(
            state.sleep_disabled, false,
            "Default state should have sleep_disabled as false"
        );
    }

    #[test]
    fn test_save_state_enabled() {
        let test_name = "save_enabled";
        let result = save_test_state(true, test_name);
        assert!(
            result.is_ok(),
            "Should successfully save state with sleep_disabled=true"
        );

        let loaded_state = load_test_state(test_name);
        assert_eq!(
            loaded_state.sleep_disabled, true,
            "Loaded state should match saved state"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_save_state_disabled() {
        let test_name = "save_disabled";
        let result = save_test_state(false, test_name);
        assert!(
            result.is_ok(),
            "Should successfully save state with sleep_disabled=false"
        );

        let loaded_state = load_test_state(test_name);
        assert_eq!(
            loaded_state.sleep_disabled, false,
            "Loaded state should match saved state"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_load_nonexistent_state() {
        let test_name = "nonexistent";
        let state = load_test_state(test_name);
        assert_eq!(
            state.sleep_disabled, false,
            "Loading nonexistent state should return default"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_state_persistence() {
        let test_name = "persistence";

        // Save initial state
        save_test_state(true, test_name).unwrap();
        let state1 = load_test_state(test_name);
        assert_eq!(state1.sleep_disabled, true);

        // Update state
        save_test_state(false, test_name).unwrap();
        let state2 = load_test_state(test_name);
        assert_eq!(state2.sleep_disabled, false, "State should persist updates");

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_state_serialization() {
        let state = AppState {
            sleep_disabled: true,
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(
            json.contains("sleep_disabled"),
            "JSON should contain sleep_disabled field"
        );
        assert!(json.contains("true"), "JSON should contain the value true");
    }

    #[test]
    fn test_state_deserialization() {
        let json = r#"{"sleep_disabled":true}"#;
        let state: AppState = serde_json::from_str(json).unwrap();
        assert_eq!(state.sleep_disabled, true);
    }

    #[test]
    fn test_state_deserialization_with_extra_fields() {
        let json = r#"{"sleep_disabled":false,"extra_field":"value"}"#;
        let state: Result<AppState, _> = serde_json::from_str(json);
        assert!(
            state.is_ok(),
            "Should deserialize successfully ignoring extra fields"
        );
    }

    #[test]
    fn test_state_file_path_creation() {
        let test_name = "path_creation";
        let path = get_test_state_file_path(test_name);

        // Ensure parent directory exists
        assert!(
            path.parent().is_some(),
            "State file should have a parent directory"
        );

        // Save something to ensure directory is created
        save_test_state(true, test_name).unwrap();
        assert!(
            path.parent().unwrap().exists(),
            "Parent directory should be created"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_multiple_state_toggles() {
        let test_name = "multiple_toggles";

        // Toggle multiple times
        for i in 0..5 {
            let should_disable = i % 2 == 0;
            save_test_state(should_disable, test_name).unwrap();
            let loaded = load_test_state(test_name);
            assert_eq!(
                loaded.sleep_disabled, should_disable,
                "State should correctly toggle on iteration {}",
                i
            );
        }

        cleanup_test_state(test_name);
    }
}

#[cfg(test)]
mod icon_tests {
    static ICON_ALLOW: &[u8] = include_bytes!("../icons/icon-allow-32x32.png");
    static ICON_BLOCK: &[u8] = include_bytes!("../icons/icon-block-32x32.png");

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

    #[test]
    fn test_icon_allow_loads() {
        assert!(
            !ICON_ALLOW.is_empty(),
            "Icon-allow data should not be empty"
        );
        assert!(
            ICON_ALLOW.len() > 100,
            "Icon-allow should have reasonable size"
        );
    }

    #[test]
    fn test_icon_block_loads() {
        assert!(
            !ICON_BLOCK.is_empty(),
            "Icon-block data should not be empty"
        );
        assert!(
            ICON_BLOCK.len() > 100,
            "Icon-block should have reasonable size"
        );
    }

    #[test]
    fn test_icon_allow_is_valid_png() {
        let result = image::load_from_memory(ICON_ALLOW);
        assert!(result.is_ok(), "Icon-allow should be a valid PNG image");
    }

    #[test]
    fn test_icon_block_is_valid_png() {
        let result = image::load_from_memory(ICON_BLOCK);
        assert!(result.is_ok(), "Icon-block should be a valid PNG image");
    }

    #[test]
    fn test_get_icon_when_disabled() {
        let result = get_icon(true);
        assert!(
            result.is_ok(),
            "Should successfully get icon for disabled state"
        );
        let icon_data = result.unwrap();
        assert!(!icon_data.is_empty(), "Icon data should not be empty");
    }

    #[test]
    fn test_get_icon_when_enabled() {
        let result = get_icon(false);
        assert!(
            result.is_ok(),
            "Should successfully get icon for enabled state"
        );
        let icon_data = result.unwrap();
        assert!(!icon_data.is_empty(), "Icon data should not be empty");
    }

    #[test]
    fn test_icon_dimensions() {
        let img_allow = image::load_from_memory(ICON_ALLOW).unwrap();
        assert_eq!(
            img_allow.width(),
            32,
            "Icon-allow width should be 32 pixels"
        );
        assert_eq!(
            img_allow.height(),
            32,
            "Icon-allow height should be 32 pixels"
        );

        let img_block = image::load_from_memory(ICON_BLOCK).unwrap();
        assert_eq!(
            img_block.width(),
            32,
            "Icon-block width should be 32 pixels"
        );
        assert_eq!(
            img_block.height(),
            32,
            "Icon-block height should be 32 pixels"
        );
    }

    #[test]
    fn test_icon_rgba_conversion() {
        let img = image::load_from_memory(ICON_ALLOW).unwrap();
        let rgba = img.into_rgba8();
        let raw = rgba.into_raw();

        // RGBA should have 4 bytes per pixel
        assert_eq!(raw.len(), 32 * 32 * 4, "RGBA data should be 32x32x4 bytes");
    }

    #[test]
    fn test_icons_are_different() {
        assert_ne!(
            ICON_ALLOW, ICON_BLOCK,
            "Allow and block icons should be different"
        );
    }

    #[test]
    fn test_get_icon_returns_different_data() {
        let icon_enabled = get_icon(false).unwrap();
        let icon_disabled = get_icon(true).unwrap();

        // The icons should produce different RGBA data
        assert_ne!(
            icon_enabled, icon_disabled,
            "Icons for different states should produce different data"
        );
    }
}

#[cfg(test)]
mod timing_tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    #[test]
    fn test_atomic_bool_operations() {
        let flag = Arc::new(AtomicBool::new(false));
        assert_eq!(flag.load(Ordering::SeqCst), false);

        flag.store(true, Ordering::SeqCst);
        assert_eq!(flag.load(Ordering::SeqCst), true);

        flag.store(false, Ordering::SeqCst);
        assert_eq!(flag.load(Ordering::SeqCst), false);
    }

    #[test]
    fn test_atomic_bool_clone() {
        let flag = Arc::new(AtomicBool::new(true));
        let flag_clone = flag.clone();

        assert_eq!(flag.load(Ordering::SeqCst), true);
        assert_eq!(flag_clone.load(Ordering::SeqCst), true);

        flag.store(false, Ordering::SeqCst);
        assert_eq!(
            flag_clone.load(Ordering::SeqCst),
            false,
            "Clone should reflect changes"
        );
    }

    #[test]
    fn test_duration_creation() {
        let duration = Duration::from_secs(60);
        assert_eq!(duration.as_secs(), 60);
        assert_eq!(duration.as_millis(), 60000);
    }

    #[tokio::test]
    async fn test_tokio_sleep() {
        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(10)).await;
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(10),
            "Should sleep for at least 10ms"
        );
        assert!(
            elapsed < Duration::from_millis(100),
            "Should not sleep for more than 100ms"
        );
    }

    #[tokio::test]
    async fn test_async_flag_checking() {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        // Spawn a task that stops after a short time
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            running_clone.store(false, Ordering::SeqCst);
        });

        let start = Instant::now();
        while running.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(50),
            "Should wait for flag to change"
        );
        assert!(
            elapsed < Duration::from_millis(150),
            "Should not wait too long"
        );
    }
}

#[cfg(test)]
mod menu_tests {
    #[test]
    fn test_menu_text_enable_sleep() {
        let text = "Enable Sleep";
        assert!(text.len() > 0);
        assert!(text.contains("Sleep"));
    }

    #[test]
    fn test_menu_text_disable_sleep() {
        let text = "Disable Sleep";
        assert!(text.len() > 0);
        assert!(text.contains("Sleep"));
    }

    #[test]
    fn test_menu_text_toggle_logic() {
        let get_menu_text = |is_disabled: bool| {
            if is_disabled {
                "Enable Sleep"
            } else {
                "Disable Sleep"
            }
        };

        assert_eq!(get_menu_text(true), "Enable Sleep");
        assert_eq!(get_menu_text(false), "Disable Sleep");
    }

    #[test]
    fn test_autostart_menu_text() {
        let get_autostart_text = |is_enabled: bool| {
            if is_enabled {
                "✓ Start at Login"
            } else {
                "Start at Login"
            }
        };

        assert_eq!(get_autostart_text(true), "✓ Start at Login");
        assert_eq!(get_autostart_text(false), "Start at Login");
    }

    #[test]
    fn test_tooltip_text() {
        let get_tooltip = |is_disabled: bool| {
            if is_disabled {
                "Awake - Sleep prevention enabled"
            } else {
                "Awake - Sleep prevention disabled"
            }
        };

        assert!(get_tooltip(true).contains("enabled"));
        assert!(get_tooltip(false).contains("disabled"));
    }
}

#[cfg(test)]
mod integration_scenarios {
    use super::*;

    #[test]
    fn test_full_enable_disable_cycle() {
        let test_name = "full_cycle";

        // Start with default state (disabled)
        let initial_state = load_test_state(test_name);
        assert_eq!(initial_state.sleep_disabled, false);

        // Enable sleep prevention
        save_test_state(true, test_name).unwrap();
        let enabled_state = load_test_state(test_name);
        assert_eq!(enabled_state.sleep_disabled, true);

        // Disable sleep prevention
        save_test_state(false, test_name).unwrap();
        let disabled_state = load_test_state(test_name);
        assert_eq!(disabled_state.sleep_disabled, false);

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_application_restart_scenario() {
        let test_name = "restart_scenario";

        // First "session" - enable sleep prevention
        save_test_state(true, test_name).unwrap();

        // Simulate app restart by loading state
        let reloaded_state = load_test_state(test_name);
        assert_eq!(
            reloaded_state.sleep_disabled, true,
            "State should persist across app restarts"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_rapid_state_changes() {
        let test_name = "rapid_changes";

        for _ in 0..10 {
            save_test_state(true, test_name).unwrap();
            save_test_state(false, test_name).unwrap();
        }

        let final_state = load_test_state(test_name);
        assert_eq!(
            final_state.sleep_disabled, false,
            "Final state should be correct after rapid changes"
        );

        cleanup_test_state(test_name);
    }

    #[test]
    fn test_concurrent_state_access() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let state = Arc::new(AtomicBool::new(false));
        let state_clone1 = state.clone();
        let state_clone2 = state.clone();

        // Simulate concurrent access
        state_clone1.store(true, Ordering::SeqCst);
        let value1 = state_clone2.load(Ordering::SeqCst);

        assert_eq!(value1, true, "Concurrent access should work correctly");
    }
}
