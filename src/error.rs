//! RustForge Migration Error Types
//!
//! Production-grade error handling for legacy migration pipelines.
//! All public functions that can fail return `Result<T, MigrationError>`.

use std::fmt;

/// Unified error type for all RustForge migration operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationError {
    /// Numerical parity check failed between legacy and Rust outputs.
    /// Contains: field name, legacy value, rust value, tolerance used.
    ParityViolation {
        field: String,
        legacy_value: f64,
        rust_value: f64,
        tolerance: f64,
    },

    /// Input data validation failed.
    InvalidInput {
        field: String,
        reason: String,
    },

    /// Computation did not converge within the allowed iterations.
    ConvergenceFailure {
        algorithm: String,
        iterations: usize,
        final_residual: f64,
        tolerance: f64,
    },

    /// COBOL source file could not be parsed.
    ParseError {
        line: usize,
        message: String,
    },

    /// General I/O error wrapping std::io::Error message.
    IoError(String),
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationError::ParityViolation { field, legacy_value, rust_value, tolerance } => {
                write!(
                    f,
                    "PARITY VIOLATION: field '{}' — legacy={:.10e}, rust={:.10e}, diff={:.2e} (tolerance={:.2e})",
                    field, legacy_value, rust_value,
                    (legacy_value - rust_value).abs(), tolerance
                )
            }
            MigrationError::InvalidInput { field, reason } => {
                write!(f, "INVALID INPUT: field '{}' — {}", field, reason)
            }
            MigrationError::ConvergenceFailure { algorithm, iterations, final_residual, tolerance } => {
                write!(
                    f,
                    "CONVERGENCE FAILURE: {} did not converge in {} iterations (residual={:.2e}, tolerance={:.2e})",
                    algorithm, iterations, final_residual, tolerance
                )
            }
            MigrationError::ParseError { line, message } => {
                write!(f, "PARSE ERROR at line {}: {}", line, message)
            }
            MigrationError::IoError(msg) => {
                write!(f, "IO ERROR: {}", msg)
            }
        }
    }
}

impl std::error::Error for MigrationError {}

impl From<std::io::Error> for MigrationError {
    fn from(e: std::io::Error) -> Self {
        MigrationError::IoError(e.to_string())
    }
}

/// Verify parity between a legacy (COBOL/Fortran) value and its Rust equivalent.
/// Returns Ok(()) if within tolerance, Err(ParityViolation) otherwise.
///
/// # Arguments
/// * `field` - Name of the field being compared (for error messages)
/// * `legacy` - Value from the legacy system
/// * `rust` - Value from the Rust implementation
/// * `tolerance` - Maximum allowed absolute difference
///
/// # Example
/// ```
/// use rustforge::error::verify_parity;
/// assert!(verify_parity("balance", 155250.0, 155250.0, 1e-10).is_ok());
/// assert!(verify_parity("balance", 155250.0, 155251.0, 1e-10).is_err());
/// ```
pub fn verify_parity(field: &str, legacy: f64, rust: f64, tolerance: f64) -> Result<(), MigrationError> {
    if (legacy - rust).abs() <= tolerance {
        Ok(())
    } else {
        Err(MigrationError::ParityViolation {
            field: field.to_string(),
            legacy_value: legacy,
            rust_value: rust,
            tolerance,
        })
    }
}

/// Validate that a financial amount is non-negative.
pub fn validate_positive(field: &str, value: f64) -> Result<(), MigrationError> {
    if value >= 0.0 {
        Ok(())
    } else {
        Err(MigrationError::InvalidInput {
            field: field.to_string(),
            reason: format!("must be non-negative, got {}", value),
        })
    }
}

/// Validate that an interest rate is in the range [0.0, 1.0].
pub fn validate_rate(field: &str, rate: f64) -> Result<(), MigrationError> {
    if (0.0..=1.0).contains(&rate) {
        Ok(())
    } else {
        Err(MigrationError::InvalidInput {
            field: field.to_string(),
            reason: format!("rate must be in [0.0, 1.0], got {:.4}", rate),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parity_ok() {
        assert!(verify_parity("balance", 155250.0, 155250.0, 1e-10).is_ok());
    }

    #[test]
    fn test_parity_within_tolerance() {
        assert!(verify_parity("balance", 155250.0, 155250.000000001, 1e-6).is_ok());
    }

    #[test]
    fn test_parity_violation() {
        let err = verify_parity("balance", 155250.0, 155251.0, 1e-10).unwrap_err();
        assert!(matches!(err, MigrationError::ParityViolation { .. }));
    }

    #[test]
    fn test_parity_error_message_contains_field() {
        let err = verify_parity("final_balance", 100.0, 200.0, 1e-10).unwrap_err();
        assert!(err.to_string().contains("final_balance"));
    }

    #[test]
    fn test_validate_positive_ok() {
        assert!(validate_positive("amount", 0.0).is_ok());
        assert!(validate_positive("amount", 100.0).is_ok());
    }

    #[test]
    fn test_validate_positive_fail() {
        let err = validate_positive("amount", -1.0).unwrap_err();
        assert!(matches!(err, MigrationError::InvalidInput { .. }));
    }

    #[test]
    fn test_validate_rate_ok() {
        assert!(validate_rate("interest_rate", 0.035).is_ok());
        assert!(validate_rate("interest_rate", 0.0).is_ok());
        assert!(validate_rate("interest_rate", 1.0).is_ok());
    }

    #[test]
    fn test_validate_rate_fail() {
        assert!(validate_rate("interest_rate", 1.5).is_err());
        assert!(validate_rate("interest_rate", -0.1).is_err());
    }

    #[test]
    fn test_error_display_convergence() {
        let err = MigrationError::ConvergenceFailure {
            algorithm: "power_iteration".to_string(),
            iterations: 100,
            final_residual: 1e-5,
            tolerance: 1e-10,
        };
        let msg = err.to_string();
        assert!(msg.contains("power_iteration"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn test_error_implements_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(MigrationError::InvalidInput {
            field: "test".to_string(),
            reason: "test reason".to_string(),
        });
        assert!(err.to_string().contains("test"));
    }
}
