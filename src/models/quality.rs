//! Quality-related data models

#[cfg(feature = "quality-proxy")]
use serde::{Deserialize, Serialize};

/// Quality assessment report containing validation results
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    /// Whether the content passed all quality checks
    pub passed: bool,
    /// List of quality violations found during assessment
    pub violations: Vec<crate::error::QualityViolation>,
    /// Suggested improvements to address quality issues
    pub suggestions: Vec<String>,
}
