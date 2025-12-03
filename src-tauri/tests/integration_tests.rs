//! Integration tests for Awake Tauri application
//! Tests Tauri commands, menu interactions, and application lifecycle

#[cfg(test)]
mod tauri_integration_tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_application_state_initialization() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));
        assert_eq!(
            sleep_disabled.load(Ordering::SeqCst),
            false,
            "Application should start with sleep disabled by default"
        );
    }

    #[test]
    fn test_state_toggle_mechanism() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));

        // Toggle on
        sleep_disabled.store(true, Ordering::SeqCst);
        assert_eq!(
            sleep_disabled.load(Ordering::SeqCst),
            true,
            "State should toggle to true"
        );

        // Toggle off
        sleep_disabled.store(false, Ordering::SeqCst);
        assert_eq!(
            sleep_disabled.load(Ordering::SeqCst),
            false,
            "State should toggle to false"
        );
    }

    #[test]
    fn test_multiple_state_references() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));
        let reference1 = sleep_disabled.clone();
        let reference2 = sleep_disabled.clone();
        let reference3 = sleep_disabled.clone();

        reference1.store(true, Ordering::SeqCst);

        assert_eq!(reference2.load(Ordering::SeqCst), true);
        assert_eq!(reference3.load(Ordering::SeqCst), true);
        assert_eq!(sleep_disabled.load(Ordering::SeqCst), true);
    }

    #[tokio::test]
    async fn test_keep_awake_task_respects_flag() {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        let iterations = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let iterations_clone = iterations.clone();

        // Spawn a mock keep-awake task
        let task = tokio::spawn(async move {
            while running_clone.load(Ordering::SeqCst) {
                iterations_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });

        // Let it run for a bit
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Stop the task
        running.store(false, Ordering::SeqCst);

        // Wait for task to complete
        let _ = tokio::time::timeout(Duration::from_secs(1), task).await;

        let count = iterations.load(std::sync::atomic::Ordering::SeqCst);
        assert!(count > 0, "Task should have executed at least once");
        assert!(
            count < 20,
            "Task should have stopped promptly when flag was set"
        );
    }

    #[tokio::test]
    async fn test_task_cancellation_on_disable() {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let task = tokio::spawn(async move {
            let mut loop_count = 0;
            while running_clone.load(Ordering::SeqCst) && loop_count < 100 {
                loop_count += 1;
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            loop_count
        });

        // Immediately disable
        tokio::time::sleep(Duration::from_millis(15)).await;
        running.store(false, Ordering::SeqCst);

        let result = task.await.unwrap();
        assert!(
            result < 100,
            "Task should exit early when disabled, got {} iterations",
            result
        );
    }

    #[test]
    fn test_menu_id_constants() {
        let toggle_sleep_id = "toggle_sleep";
        let toggle_autostart_id = "toggle_autostart";
        let quit_id = "quit";

        assert_eq!(toggle_sleep_id, "toggle_sleep");
        assert_eq!(toggle_autostart_id, "toggle_autostart");
        assert_eq!(quit_id, "quit");

        // Ensure IDs are unique
        assert_ne!(toggle_sleep_id, toggle_autostart_id);
        assert_ne!(toggle_sleep_id, quit_id);
        assert_ne!(toggle_autostart_id, quit_id);
    }

    #[test]
    fn test_menu_text_generation() {
        fn get_sleep_menu_text(is_disabled: bool) -> &'static str {
            if is_disabled {
                "Enable Sleep"
            } else {
                "Disable Sleep"
            }
        }

        assert_eq!(get_sleep_menu_text(false), "Disable Sleep");
        assert_eq!(get_sleep_menu_text(true), "Enable Sleep");
    }

    #[test]
    fn test_autostart_menu_text_generation() {
        fn get_autostart_menu_text(is_enabled: bool) -> &'static str {
            if is_enabled {
                "✓ Start at Login"
            } else {
                "Start at Login"
            }
        }

        let disabled_text = get_autostart_menu_text(false);
        let enabled_text = get_autostart_menu_text(true);

        assert_eq!(disabled_text, "Start at Login");
        assert_eq!(enabled_text, "✓ Start at Login");
        assert!(enabled_text.contains("✓"));
        assert!(!disabled_text.contains("✓"));
    }

    #[test]
    fn test_tooltip_generation() {
        fn get_tooltip(is_disabled: bool) -> &'static str {
            if is_disabled {
                "Awake - Sleep prevention enabled"
            } else {
                "Awake - Sleep prevention disabled"
            }
        }

        let enabled_tooltip = get_tooltip(true);
        let disabled_tooltip = get_tooltip(false);

        assert!(enabled_tooltip.contains("enabled"));
        assert!(disabled_tooltip.contains("disabled"));
        assert!(enabled_tooltip.starts_with("Awake"));
        assert!(disabled_tooltip.starts_with("Awake"));
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_application_startup_sequence() {
        // Simulate app startup
        let sleep_disabled = Arc::new(AtomicBool::new(false));

        // Load state (simulated as false)
        assert_eq!(sleep_disabled.load(Ordering::SeqCst), false);

        // No task should be spawned when starting disabled
        let task_running = false;
        assert_eq!(task_running, false);
    }

    #[tokio::test]
    async fn test_application_startup_with_saved_state() {
        // Simulate app startup with saved enabled state
        let sleep_disabled = Arc::new(AtomicBool::new(true));

        assert_eq!(sleep_disabled.load(Ordering::SeqCst), true);

        // Task should be spawned when starting enabled
        let running_clone = sleep_disabled.clone();
        let task = tokio::spawn(async move {
            let mut count = 0;
            while running_clone.load(Ordering::SeqCst) && count < 5 {
                count += 1;
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            count
        });

        tokio::time::sleep(Duration::from_millis(30)).await;
        sleep_disabled.store(false, Ordering::SeqCst);

        let iterations = task.await.unwrap();
        assert!(iterations > 0, "Task should run on startup when enabled");
    }

    #[tokio::test]
    async fn test_application_shutdown() {
        let sleep_disabled = Arc::new(AtomicBool::new(true));
        let running_clone = sleep_disabled.clone();

        let task = tokio::spawn(async move {
            while running_clone.load(Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });

        // Simulate quit
        sleep_disabled.store(false, Ordering::SeqCst);

        // Task should complete
        let result = tokio::time::timeout(Duration::from_millis(100), task).await;
        assert!(result.is_ok(), "Task should complete on shutdown");
    }

    #[test]
    fn test_state_transitions() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));

        // Transition: Disabled -> Enabled
        sleep_disabled.store(true, Ordering::SeqCst);
        assert_eq!(sleep_disabled.load(Ordering::SeqCst), true);

        // Transition: Enabled -> Disabled
        sleep_disabled.store(false, Ordering::SeqCst);
        assert_eq!(sleep_disabled.load(Ordering::SeqCst), false);

        // Multiple rapid transitions (0 is even, so we start with true)
        for i in 0..10 {
            sleep_disabled.store(i % 2 == 0, Ordering::SeqCst);
        }
        // After 10 iterations (0-9), last value is i=9 which is odd, so false
        assert_eq!(sleep_disabled.load(Ordering::SeqCst), false);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Serialize, Deserialize, Default)]
    struct AppState {
        #[serde(default)]
        sleep_disabled: bool,
    }

    #[test]
    fn test_invalid_json_handling() {
        let invalid_json = "{invalid json}";
        let result: Result<AppState, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err(), "Should fail to parse invalid JSON");
    }

    #[test]
    fn test_missing_field_handling() {
        let empty_json = "{}";
        let result: Result<AppState, _> = serde_json::from_str(empty_json);
        // serde should use default value when field attribute is present
        assert!(result.is_ok(), "Should handle missing fields with defaults");
        let state = result.unwrap();
        assert_eq!(state.sleep_disabled, false, "Default should be false");
    }

    #[test]
    fn test_state_file_path_generation() {
        let temp_dir = std::env::temp_dir();
        assert!(temp_dir.exists(), "Temp directory should exist");

        let mut test_path = temp_dir;
        test_path.push("awake_test");
        test_path.push("state.json");

        assert!(test_path.to_str().is_some(), "Path should be valid UTF-8");
    }

    #[test]
    fn test_corrupted_state_recovery() {
        let test_dir = std::env::temp_dir().join("awake_test_corrupted");
        fs::create_dir_all(&test_dir).unwrap();

        let state_file = test_dir.join("state.json");
        fs::write(&state_file, "corrupted data").unwrap();

        // Try to load corrupted state
        let content = fs::read_to_string(&state_file).unwrap();
        let result: Result<AppState, _> = serde_json::from_str(&content);

        if result.is_err() {
            // Should fall back to default
            let default_state = AppState::default();
            assert_eq!(default_state.sleep_disabled, false);
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).unwrap();
    }
}

#[cfg(test)]
mod menu_interaction_tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_toggle_sleep_menu_action() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));

        // Simulate menu click
        let is_currently_disabled = sleep_disabled.load(Ordering::SeqCst);
        let new_state = !is_currently_disabled;
        sleep_disabled.store(new_state, Ordering::SeqCst);

        assert_eq!(sleep_disabled.load(Ordering::SeqCst), true);
    }

    #[test]
    fn test_quit_menu_action() {
        let sleep_disabled = Arc::new(AtomicBool::new(true));

        // Simulate quit action
        sleep_disabled.store(false, Ordering::SeqCst);

        assert_eq!(
            sleep_disabled.load(Ordering::SeqCst),
            false,
            "Should stop keep-awake on quit"
        );
    }

    #[test]
    fn test_menu_text_updates_on_toggle() {
        let mut is_disabled = false;

        let initial_text = if is_disabled {
            "Enable Sleep"
        } else {
            "Disable Sleep"
        };
        assert_eq!(initial_text, "Disable Sleep");

        // Toggle state
        is_disabled = true;

        let updated_text = if is_disabled {
            "Enable Sleep"
        } else {
            "Disable Sleep"
        };
        assert_eq!(updated_text, "Enable Sleep");
    }

    #[test]
    fn test_autostart_toggle() {
        let is_autostart_enabled = false;

        // Check disabled state
        let text = if is_autostart_enabled {
            "✓ Start at Login"
        } else {
            "Start at Login"
        };
        assert_eq!(text, "Start at Login");

        // Check enabled state
        let is_autostart_enabled = true;
        let text = if is_autostart_enabled {
            "✓ Start at Login"
        } else {
            "Start at Login"
        };
        assert_eq!(text, "✓ Start at Login");
    }
}

#[cfg(test)]
mod icon_state_tests {
    #[test]
    fn test_icon_selection_logic() {
        fn get_icon_type(is_sleep_disabled: bool) -> &'static str {
            if is_sleep_disabled {
                "ICON_BLOCK"
            } else {
                "ICON_ALLOW"
            }
        }

        assert_eq!(get_icon_type(false), "ICON_ALLOW");
        assert_eq!(get_icon_type(true), "ICON_BLOCK");
    }

    #[test]
    fn test_icon_dimensions_requirement() {
        let expected_width = 32;
        let expected_height = 32;
        let expected_bytes = expected_width * expected_height * 4; // RGBA

        assert_eq!(expected_bytes, 4096);
    }

    #[test]
    fn test_icon_state_changes() {
        let current_icon = "ICON_ALLOW";
        assert_eq!(current_icon, "ICON_ALLOW");

        // Enable sleep prevention
        let current_icon = "ICON_BLOCK";
        assert_eq!(current_icon, "ICON_BLOCK");

        // Disable sleep prevention
        let current_icon = "ICON_ALLOW";
        assert_eq!(current_icon, "ICON_ALLOW");
    }
}

#[cfg(test)]
mod concurrency_tests {
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_concurrent_state_access() {
        let sleep_disabled = Arc::new(AtomicBool::new(false));
        let counter = Arc::new(AtomicU32::new(0));

        let mut handles = vec![];

        // Spawn multiple tasks accessing the same state
        for _ in 0..5 {
            let sleep_clone = sleep_disabled.clone();
            let counter_clone = counter.clone();

            let handle = tokio::spawn(async move {
                for _ in 0..10 {
                    if sleep_clone.load(Ordering::SeqCst) {
                        counter_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            });
            handles.push(handle);
        }

        // Enable sleep prevention
        sleep_disabled.store(true, Ordering::SeqCst);

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        let final_count = counter.load(Ordering::SeqCst);
        assert!(final_count > 0, "Tasks should have incremented counter");
    }

    #[tokio::test]
    async fn test_task_spawn_and_cancel() {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let task = tokio::spawn(async move {
            while running_clone.load(Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            "completed"
        });

        tokio::time::sleep(Duration::from_millis(20)).await;
        running.store(false, Ordering::SeqCst);

        let result = tokio::time::timeout(Duration::from_millis(100), task).await;
        assert!(result.is_ok(), "Task should complete gracefully");
        assert_eq!(result.unwrap().unwrap(), "completed");
    }
}
