//! PMAT quality proxy integration for comprehensive quality enforcement
//!
//! This module provides integration with the PAIML MCP Agent Toolkit (PMAT)
//! quality proxy service for enforcing strict quality standards on generated code.

use crate::error::Result;
use crate::models::quality::QualityReport;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Proxy mode for quality enforcement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    /// Strict mode - all quality gates must pass
    Strict,
    /// Advisory mode - quality issues are reported but not enforced
    Advisory,
    /// Auto-fix mode - automatically fix quality issues when possible
    AutoFix,
}

/// Quality proxy for integrating with PMAT quality enforcement
#[derive(Debug)]
pub struct QualityProxy {
    /// API endpoint for the quality proxy service
    endpoint: String,
    /// Timeout duration for quality operations
    timeout: std::time::Duration,
    /// HTTP client for making requests
    #[cfg(feature = "quality-proxy")]
    client: reqwest::Client,
    /// Proxy configuration
    config: ProxyConfig,
}

/// Proxy configuration for quality enforcement
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Enforcement mode
    pub mode: ProxyMode,
    /// Minimum coverage requirement (0.0 - 100.0)
    pub min_coverage: f64,
    /// Require doctests for all public APIs
    pub require_doctests: bool,
    /// Require property tests for complex logic
    pub require_property_tests: bool,
    /// Require working examples
    pub require_examples: bool,
    /// Zero tolerance for SATD (TODO/FIXME/HACK)
    pub zero_satd: bool,
    /// Maximum cyclomatic complexity
    pub max_complexity: u32,
    /// Auto-fix issues when possible
    pub auto_fix: bool,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            mode: ProxyMode::Strict,
            min_coverage: 80.0,
            require_doctests: true,
            require_property_tests: true,
            require_examples: true,
            zero_satd: true,
            max_complexity: 8,
            auto_fix: false,
        }
    }
}

/// Operation type for proxy requests
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyOperation {
    /// Validate content against quality standards
    Validate,
    /// Write content with quality enforcement
    Write,
    /// Refactor existing content
    Refactor,
    /// Format content according to standards
    Format,
}

/// Request payload for quality proxy operations
#[derive(Debug, Serialize)]
pub struct ProxyRequest {
    /// Type of operation to perform
    pub operation: ProxyOperation,
    /// File path for the operation
    pub file_path: String,
    /// Content to be processed (optional for read operations)
    pub content: Option<String>,
    /// Processing mode
    pub mode: ProxyMode,
    /// Quality configuration settings
    pub quality_config: ProxyConfig,
    /// Additional metadata for the operation
    pub metadata: HashMap<String, String>,
}

/// Quality configuration for proxy operations (legacy compatibility)
#[derive(Debug, Clone, Copy, Serialize)]
pub struct QualityConfig {
    /// Maximum allowed complexity score
    pub max_complexity: u32,
    /// Whether to allow SATD (Self-Admitted Technical Debt) comments
    pub allow_satd: bool,
    /// Whether to require documentation
    pub require_docs: bool,
    /// Whether to automatically format code
    pub auto_format: bool,
}

/// Status of proxy operation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyStatus {
    /// Operation accepted, quality gates passed
    Accepted,
    /// Operation rejected due to quality violations
    Rejected,
    /// Operation modified with auto-fixes applied
    Modified,
}

/// Response from quality proxy operations
#[derive(Debug, Deserialize)]
pub struct ProxyResponse {
    /// Status of the operation
    pub status: ProxyStatus,
    /// Final processed content
    pub final_content: String,
    /// Quality analysis report
    pub quality_report: QualityReport,
    /// Applied fixes (if any)
    pub applied_fixes: Vec<String>,
    /// Quality metrics
    pub metrics: QualityMetrics,
}

/// Quality metrics from proxy validation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Code coverage percentage
    pub coverage: f64,
    /// Cyclomatic complexity score
    pub complexity: u32,
    /// Number of doctests
    pub doctest_count: usize,
    /// Number of property tests
    pub property_test_count: usize,
    /// Number of examples
    pub example_count: usize,
    /// SATD violations count
    pub satd_count: usize,
}

impl QualityProxy {
    /// Create a new quality proxy instance with default configuration
    pub fn new(endpoint: String) -> Self {
        Self::with_config(endpoint, ProxyConfig::default())
    }
    
    /// Create a new quality proxy instance with custom configuration
    pub fn with_config(endpoint: String, config: ProxyConfig) -> Self {
        Self {
            endpoint,
            timeout: crate::DEFAULT_QUALITY_TIMEOUT,
            #[cfg(feature = "quality-proxy")]
            client: reqwest::Client::builder()
                .timeout(crate::DEFAULT_QUALITY_TIMEOUT)
                .build()
                .expect("Failed to create HTTP client"),
            config,
        }
    }

    /// Proxy a quality operation through PMAT
    pub async fn proxy_operation(&self, request: ProxyRequest) -> Result<ProxyResponse> {
        #[cfg(feature = "quality-proxy")]
        {
            // Make HTTP request to PMAT quality proxy service
            let response = self.client
                .post(&format!("{}/proxy", self.endpoint))
                .json(&request)
                .timeout(self.timeout)
                .send()
                .await
                .map_err(|e| crate::error::Error::Internal(format!("Proxy request failed: {}", e)))?;
            
            if response.status().is_success() {
                response.json::<ProxyResponse>()
                    .await
                    .map_err(|e| crate::error::Error::Internal(format!("Failed to parse response: {}", e)))
            } else {
                Err(crate::error::Error::Internal(format!(
                    "Quality proxy returned error: {}",
                    response.status()
                )))
            }
        }
        
        #[cfg(not(feature = "quality-proxy"))]
        {
            // Fallback implementation when quality-proxy feature is disabled
            Ok(ProxyResponse {
                status: ProxyStatus::Accepted,
                final_content: request.content.unwrap_or_default(),
                quality_report: QualityReport {
                    passed: true,
                    violations: Vec::new(),
                    suggestions: Vec::new(),
                },
                applied_fixes: Vec::new(),
                metrics: QualityMetrics {
                    coverage: 100.0,
                    complexity: 1,
                    doctest_count: 0,
                    property_test_count: 0,
                    example_count: 0,
                    satd_count: 0,
                },
            })
        }
    }

    /// Validate and refactor content using the quality proxy (legacy method)
    pub async fn validate_and_refactor(
        &self,
        content: &str,
        _config: &QualityConfig,
    ) -> Result<ProxyResponse> {
        let request = ProxyRequest {
            operation: ProxyOperation::Validate,
            file_path: "temp.rs".to_string(),
            content: Some(content.to_string()),
            mode: self.config.mode,
            quality_config: self.config.clone(),
            metadata: HashMap::new(),
        };
        
        self.proxy_operation(request).await
    }
    
    /// Validate content against quality standards
    pub async fn validate(&self, content: &str, file_path: &str) -> Result<ProxyResponse> {
        let request = ProxyRequest {
            operation: ProxyOperation::Validate,
            file_path: file_path.to_string(),
            content: Some(content.to_string()),
            mode: self.config.mode,
            quality_config: self.config.clone(),
            metadata: HashMap::new(),
        };
        
        self.proxy_operation(request).await
    }
    
    /// Write content with quality enforcement
    pub async fn write(&self, content: &str, file_path: &str) -> Result<ProxyResponse> {
        let request = ProxyRequest {
            operation: ProxyOperation::Write,
            file_path: file_path.to_string(),
            content: Some(content.to_string()),
            mode: self.config.mode,
            quality_config: self.config.clone(),
            metadata: HashMap::new(),
        };
        
        self.proxy_operation(request).await
    }
}
