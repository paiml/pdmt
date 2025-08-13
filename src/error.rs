//! Error handling for the PDMT library
//!
//! This module provides a comprehensive error system that covers all aspects
//! of template processing, quality validation, and MCP integration.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main error type for PDMT operations
#[derive(Error, Debug)]
pub enum Error {
    /// Template-related errors
    #[error("Template error: {0}")]
    Template(#[from] TemplateError),

    /// Quality validation errors
    #[cfg(feature = "quality-proxy")]
    #[error("Quality validation error: {0}")]
    Quality(#[from] QualityError),

    /// MCP integration errors
    #[cfg(feature = "mcp-tools")]
    #[error("MCP error: {0}")]
    Mcp(#[from] McpError),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid input errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Internal errors that shouldn't normally occur
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Template-specific errors
#[derive(Error, Debug)]
pub enum TemplateError {
    /// Template not found
    #[error("Template '{id}' not found")]
    NotFound {
        /// Template identifier that was not found
        id: String,
    },

    /// Template compilation failed
    #[error("Template compilation failed: {message}")]
    CompilationFailed {
        /// Error message from compilation failure
        message: String,
    },

    /// Template rendering failed
    #[error("Template rendering failed: {message}")]
    RenderingFailed {
        /// Error message from rendering failure
        message: String,
    },

    /// Invalid template definition
    #[error("Invalid template definition: {reason}")]
    InvalidDefinition {
        /// Reason for invalid definition
        reason: String,
    },

    /// Template inheritance error
    #[error("Template inheritance error: {message}")]
    InheritanceError {
        /// Error message
        message: String,
    },

    /// Schema validation failed
    #[error("Schema validation failed: {errors:?}")]
    SchemaValidation {
        /// List of validation errors
        errors: Vec<String>,
    },

    /// Template size limit exceeded
    #[error("Template size {size} exceeds limit {limit}")]
    SizeLimit {
        /// Actual size
        size: usize,
        /// Maximum allowed size
        limit: usize,
    },
}

/// Quality validation errors
#[cfg(feature = "quality-proxy")]
#[derive(Error, Debug)]
pub enum QualityError {
    /// Quality gate failed
    #[error("Quality gate failed: {violations:?}")]
    QualityGateFailed {
        /// List of quality violations
        violations: Vec<QualityViolation>,
        /// Improvement suggestions
        suggestions: Vec<String>,
    },

    /// Quality proxy unavailable
    #[error("Quality proxy unavailable: {reason}")]
    ProxyUnavailable {
        /// Reason for unavailability
        reason: String,
    },

    /// Quality validation timeout
    #[error("Quality validation timeout after {duration:?}")]
    Timeout {
        /// Timeout duration
        duration: std::time::Duration,
    },

    /// Invalid quality configuration
    #[error("Invalid quality configuration: {reason}")]
    InvalidConfig {
        /// Configuration error reason
        reason: String,
    },

    /// Unknown quality response
    #[error("Unknown quality response status: {status}")]
    UnknownResponse {
        /// Response status
        status: String,
    },
}

/// MCP integration errors
#[cfg(feature = "mcp-tools")]
#[derive(Error, Debug)]
pub enum McpError {
    /// Invalid MCP request
    #[error("Invalid MCP request: {message}")]
    InvalidRequest {
        /// Request error message
        message: String,
    },

    /// MCP tool not found
    #[error("MCP tool '{name}' not found")]
    ToolNotFound {
        /// Tool name
        name: String,
    },

    /// MCP protocol error
    #[error("MCP protocol error: {message}")]
    Protocol {
        /// Protocol error message
        message: String,
    },

    /// MCP transport error
    #[error("MCP transport error: {message}")]
    Transport {
        /// Transport error message
        message: String,
    },

    /// MCP timeout
    #[error("MCP operation timeout after {duration:?}")]
    Timeout {
        /// Timeout duration
        duration: std::time::Duration,
    },
}

/// Content validation errors
#[derive(Error, Debug)]
pub enum ValidationError {
    /// Required field missing
    #[error("Required field '{field}' is missing")]
    MissingField {
        /// Field name
        field: String,
    },

    /// Invalid field value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue {
        /// Field name
        field: String,
        /// Validation failure reason
        reason: String,
    },

    /// Structure validation failed
    #[error("Structure validation failed: {reason}")]
    StructureError {
        /// Structure error reason
        reason: String,
    },

    /// Todo-specific validation errors
    #[cfg(feature = "todo-validation")]
    #[error("Todo validation error: {0}")]
    Todo(#[from] TodoValidationError),

    /// Cross-reference validation failed
    #[error("Cross-reference validation failed: {message}")]
    CrossReference {
        /// Cross-reference error message
        message: String,
    },

    /// Constraint violation
    #[error("Constraint violated: {constraint} - {details}")]
    Constraint {
        /// Constraint name
        constraint: String,
        /// Violation details
        details: String,
    },
}

/// Todo-specific validation errors
#[cfg(feature = "todo-validation")]
#[derive(Error, Debug)]
pub enum TodoValidationError {
    /// Todo is not actionable
    #[error("Todo '{content}' is not actionable")]
    NotActionable {
        /// Todo content
        content: String,
    },

    /// Todo is too vague
    #[error("Todo '{content}' is too vague (min {min_chars} chars)")]
    TooVague {
        /// Todo content
        content: String,
        /// Minimum required characters
        min_chars: usize,
    },

    /// Missing time estimate
    #[error("Todo '{id}' missing time estimate")]
    MissingEstimate {
        /// Todo ID
        id: String,
    },

    /// Circular dependency detected
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency {
        /// Dependency cycle
        cycle: Vec<String>,
    },

    /// Invalid priority
    #[error("Invalid priority '{priority}' for todo '{id}'")]
    InvalidPriority {
        /// Todo ID
        id: String,
        /// Invalid priority value
        priority: String,
    },

    /// Todo count exceeds limit
    #[error("Todo count {count} exceeds limit {limit}")]
    CountLimit {
        /// Actual count
        count: usize,
        /// Maximum allowed
        limit: usize,
    },

    /// Dependency not found
    #[error("Dependency '{dependency}' not found for todo '{id}'")]
    DependencyNotFound {
        /// Todo ID
        id: String,
        /// Missing dependency ID
        dependency: String,
    },
}

/// Quality violation details
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityViolation {
    /// Type of violation (complexity, satd, lint, docs)
    pub violation_type: String,
    /// Severity level
    pub severity: Severity,
    /// Location in content (`file:line:column`)
    pub location: Option<String>,
    /// Human-readable message
    pub message: String,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Severity levels for quality violations
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Info - nice to fix
    Info,
}

/// Result type alias for PDMT operations
pub type Result<T> = std::result::Result<T, Error>;

// Implement From traits for common error conversions
impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<handlebars::RenderError> for TemplateError {
    fn from(err: handlebars::RenderError) -> Self {
        Self::RenderingFailed {
            message: err.to_string(),
        }
    }
}

impl From<handlebars::TemplateError> for TemplateError {
    fn from(err: handlebars::TemplateError) -> Self {
        Self::CompilationFailed {
            message: err.to_string(),
        }
    }
}

#[cfg(feature = "quality-proxy")]
impl From<reqwest::Error> for QualityError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::Timeout {
                duration: std::time::Duration::from_secs(30), // Default timeout
            }
        } else {
            Self::ProxyUnavailable {
                reason: err.to_string(),
            }
        }
    }
}

/// Helper functions for creating common errors
impl Error {
    /// Create an invalid input error
    pub fn invalid_input<S: Into<String>>(message: S) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Create a configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create an internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal(message.into())
    }
}

impl TemplateError {
    /// Create a template not found error
    pub fn not_found<S: Into<String>>(id: S) -> Self {
        Self::NotFound { id: id.into() }
    }

    /// Create an invalid definition error
    pub fn invalid_definition<S: Into<String>>(reason: S) -> Self {
        Self::InvalidDefinition {
            reason: reason.into(),
        }
    }

    /// Create a size limit error
    #[must_use]
    pub const fn size_limit(size: usize, limit: usize) -> Self {
        Self::SizeLimit { size, limit }
    }
}

impl ValidationError {
    /// Create a missing field error
    pub fn missing_field<S: Into<String>>(field: S) -> Self {
        ValidationError::MissingField {
            field: field.into(),
        }
    }

    /// Create an invalid value error
    pub fn invalid_value<S: Into<String>>(field: S, reason: S) -> Self {
        ValidationError::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Create a structure error
    pub fn structure<S: Into<String>>(reason: S) -> Self {
        ValidationError::StructureError {
            reason: reason.into(),
        }
    }
}

#[cfg(feature = "quality-proxy")]
impl QualityViolation {
    /// Create a new quality violation
    pub fn new<S: Into<String>>(violation_type: S, severity: Severity, message: S) -> Self {
        Self {
            violation_type: violation_type.into(),
            severity,
            location: None,
            message: message.into(),
            suggestion: None,
        }
    }

    /// Add location information
    pub fn with_location<S: Into<String>>(mut self, location: S) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Add suggestion
    pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::invalid_input("test message");
        assert!(matches!(err, Error::InvalidInput(_)));

        let err = TemplateError::not_found("test_template");
        assert!(matches!(err, TemplateError::NotFound { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = Error::invalid_input("test input error");
        assert!(err.to_string().contains("test input error"));

        let err = TemplateError::not_found("missing_template");
        assert!(err.to_string().contains("missing_template"));
    }

    #[cfg(feature = "quality-proxy")]
    #[test]
    fn test_quality_violation() {
        let violation = QualityViolation::new("complexity", Severity::Error, "Too complex")
            .with_location("file.rs:10:5")
            .with_suggestion("Split into smaller functions");

        assert_eq!(violation.violation_type, "complexity");
        assert_eq!(violation.severity, Severity::Error);
        assert_eq!(violation.location, Some("file.rs:10:5".to_string()));
    }
}
