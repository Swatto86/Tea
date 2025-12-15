//! Application error types
//!
//! Provides structured, explicit error handling for all fallible operations.
//! Errors include human-readable messages, technical causes, and recovery hints.

use std::fmt;

/// Application-wide error type
///
/// All fallible operations return this type. No `unwrap()`, `expect()`, or
/// silent failures are permitted.
#[derive(Debug)]
pub enum AppError {
    /// Failed to perform I/O operation on state file
    StateIo {
        message: String,
        cause: String,
        recovery_hint: &'static str,
    },
    /// Failed to serialize or deserialize state
    StateSerialization {
        message: String,
        cause: String,
        recovery_hint: &'static str,
    },
    /// Failed to load or process icon data
    IconProcessing {
        message: String,
        cause: String,
        recovery_hint: &'static str,
    },
    /// Failed to initialize input simulation
    InputSimulation {
        message: String,
        cause: String,
        recovery_hint: &'static str,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::StateIo {
                message,
                cause,
                recovery_hint,
            } => write!(
                f,
                "State I/O error: {} (cause: {}, hint: {})",
                message, cause, recovery_hint
            ),
            AppError::StateSerialization {
                message,
                cause,
                recovery_hint,
            } => write!(
                f,
                "State serialization error: {} (cause: {}, hint: {})",
                message, cause, recovery_hint
            ),
            AppError::IconProcessing {
                message,
                cause,
                recovery_hint,
            } => write!(
                f,
                "Icon processing error: {} (cause: {}, hint: {})",
                message, cause, recovery_hint
            ),
            AppError::InputSimulation {
                message,
                cause,
                recovery_hint,
            } => write!(
                f,
                "Input simulation error: {} (cause: {}, hint: {})",
                message, cause, recovery_hint
            ),
        }
    }
}

impl std::error::Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
