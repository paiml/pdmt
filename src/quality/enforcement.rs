//! Quality enforcement implementation for PDMT
//!
//! This module provides the core quality enforcement logic that integrates
//! with PMAT quality proxy to ensure all generated code meets strict standards.

use crate::error::Result;
use crate::models::todo::{Todo, TodoList};
use crate::quality::proxy::{ProxyConfig, ProxyOperation, ProxyRequest, QualityProxy};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for quality enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementConfig {
    /// Proxy configuration
    pub proxy_config: ProxyConfig,
    /// Enable coverage validation
    pub validate_coverage: bool,
    /// Enable doctest validation
    pub validate_doctests: bool,
    /// Enable property test validation
    pub validate_property_tests: bool,
    /// Enable example validation
    pub validate_examples: bool,
    /// Enable SATD detection
    pub detect_satd: bool,
    /// Enable complexity analysis
    pub analyze_complexity: bool,
}

impl Default for EnforcementConfig {
    fn default() -> Self {
        Self {
            proxy_config: ProxyConfig::default(),
            validate_coverage: true,
            validate_doctests: true,
            validate_property_tests: true,
            validate_examples: true,
            detect_satd: true,
            analyze_complexity: true,
        }
    }
}

/// Result of quality enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementResult {
    /// All quality gates passed
    AllPassed {
        /// Quality metrics
        metrics: HashMap<String, f64>,
        /// Applied fixes
        fixes: Vec<String>,
    },
    /// Some quality gates failed
    Failed {
        /// Failed quality gates
        failures: Vec<QualityFailure>,
        /// Suggestions for fixes
        suggestions: Vec<String>,
    },
    /// Quality gates passed with warnings
    PassedWithWarnings {
        /// Warning messages
        warnings: Vec<String>,
        /// Quality metrics
        metrics: HashMap<String, f64>,
    },
}

/// Quality failure information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityFailure {
    /// Gate that failed
    pub gate: String,
    /// Failure message
    pub message: String,
    /// Severity level
    pub severity: FailureSeverity,
    /// File path (if applicable)
    pub file_path: Option<String>,
    /// Line number (if applicable)
    pub line_number: Option<usize>,
}

/// Severity of quality failure
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FailureSeverity {
    /// Informational - doesn't block
    Info,
    /// Warning - should be addressed
    Warning,
    /// Error - must be fixed
    Error,
    /// Critical - immediate action required
    Critical,
}

/// Quality enforcer for PDMT with PMAT integration
#[derive(Debug)]
pub struct QualityEnforcer {
    /// Quality proxy instance
    proxy: QualityProxy,
    /// Enforcement configuration
    config: EnforcementConfig,
    /// Cached validation results
    _cache: HashMap<String, EnforcementResult>,
}

impl QualityEnforcer {
    /// Create a new quality enforcer
    pub fn new(proxy_endpoint: String) -> Self {
        Self::with_config(proxy_endpoint, EnforcementConfig::default())
    }
    
    /// Create a new quality enforcer with custom configuration
    pub fn with_config(proxy_endpoint: String, config: EnforcementConfig) -> Self {
        let proxy = QualityProxy::with_config(proxy_endpoint, config.proxy_config.clone());
        Self {
            proxy,
            config,
            _cache: HashMap::new(),
        }
    }
    
    /// Enforce quality standards on a todo list
    pub async fn enforce_todo_quality(&mut self, todo_list: &TodoList) -> Result<EnforcementResult> {
        let mut failures = Vec::new();
        let warnings = Vec::new();
        let mut metrics = HashMap::new();
        
        // Validate each todo
        for todo in &todo_list.todos {
            if let Err(failure) = self.validate_todo(todo).await {
                failures.push(failure);
            }
        }
        
        // TODO: Check for circular dependencies when TodoList supports it
        
        // Calculate quality metrics
        let total_todos = todo_list.todos.len();
        let actionable_todos = todo_list.todos.iter()
            .filter(|t| Self::is_actionable(&t.content))
            .count();
        
        metrics.insert("total_todos".to_string(), total_todos as f64);
        metrics.insert("actionable_ratio".to_string(), 
            if total_todos > 0 { actionable_todos as f64 / total_todos as f64 } else { 0.0 });
        
        // Determine result
        if failures.is_empty() {
            if warnings.is_empty() {
                Ok(EnforcementResult::AllPassed {
                    metrics,
                    fixes: Vec::new(),
                })
            } else {
                Ok(EnforcementResult::PassedWithWarnings {
                    warnings,
                    metrics,
                })
            }
        } else {
            let suggestions = self.generate_suggestions(&failures);
            Ok(EnforcementResult::Failed {
                failures,
                suggestions,
            })
        }
    }
    
    /// Validate a single todo
    async fn validate_todo(&self, todo: &Todo) -> std::result::Result<(), QualityFailure> {
        // Check actionability
        if !Self::is_actionable(&todo.content) {
            return Err(QualityFailure {
                gate: "actionability".to_string(),
                message: format!("Todo '{}' does not start with an action verb", todo.content),
                severity: FailureSeverity::Error,
                file_path: None,
                line_number: None,
            });
        }
        
        // Check time estimate
        if let Some(hours) = todo.estimated_hours {
            if hours < 0.5 || hours > 40.0 {
                return Err(QualityFailure {
                    gate: "time_estimation".to_string(),
                    message: format!("Unrealistic time estimate: {} hours", hours),
                    severity: FailureSeverity::Warning,
                    file_path: None,
                    line_number: None,
                });
            }
        }
        
        // Check content length
        if todo.content.len() < 10 {
            return Err(QualityFailure {
                gate: "content_validation".to_string(),
                message: "Todo content too short (minimum 10 characters)".to_string(),
                severity: FailureSeverity::Error,
                file_path: None,
                line_number: None,
            });
        }
        
        if todo.content.len() > 100 {
            return Err(QualityFailure {
                gate: "content_validation".to_string(),
                message: "Todo content too long (maximum 100 characters)".to_string(),
                severity: FailureSeverity::Warning,
                file_path: None,
                line_number: None,
            });
        }
        
        Ok(())
    }
    
    /// Check if content is actionable
    fn is_actionable(content: &str) -> bool {
        const ACTION_VERBS: &[&str] = &[
            "implement", "create", "build", "fix", "update", "add", "remove",
            "refactor", "optimize", "test", "document", "review", "deploy",
            "configure", "setup", "install", "integrate", "validate", "verify",
            "analyze", "design", "develop", "enhance", "improve", "migrate",
        ];
        
        let lower = content.to_lowercase();
        ACTION_VERBS.iter().any(|verb| lower.starts_with(verb))
    }
    
    /// Generate suggestions for fixing failures
    fn generate_suggestions(&self, failures: &[QualityFailure]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for failure in failures {
            match failure.gate.as_str() {
                "actionability" => {
                    suggestions.push(format!(
                        "Start todo with an action verb like 'Implement', 'Create', or 'Fix'"
                    ));
                }
                "time_estimation" => {
                    suggestions.push(format!(
                        "Adjust time estimate to be between 0.5 and 40 hours"
                    ));
                }
                "content_validation" => {
                    if failure.message.contains("short") {
                        suggestions.push("Add more specific details to the todo".to_string());
                    } else {
                        suggestions.push("Break down complex todo into smaller tasks".to_string());
                    }
                }
                "dependency_validation" => {
                    suggestions.push("Review and fix circular dependencies between tasks".to_string());
                }
                _ => {}
            }
        }
        
        suggestions
    }
    
    /// Enforce quality on generated code
    pub async fn enforce_code_quality(
        &mut self,
        code: &str,
        file_path: &str,
    ) -> Result<EnforcementResult> {
        // Create proxy request
        let request = ProxyRequest {
            operation: ProxyOperation::Validate,
            file_path: file_path.to_string(),
            content: Some(code.to_string()),
            mode: self.config.proxy_config.mode,
            quality_config: self.config.proxy_config.clone(),
            metadata: HashMap::new(),
        };
        
        // Send to proxy
        let response = self.proxy.proxy_operation(request).await?;
        
        // Process response
        use crate::quality::proxy::ProxyStatus;
        match response.status {
            ProxyStatus::Accepted => {
                let mut metrics = HashMap::new();
                metrics.insert("coverage".to_string(), response.metrics.coverage);
                metrics.insert("complexity".to_string(), response.metrics.complexity as f64);
                metrics.insert("doctest_count".to_string(), response.metrics.doctest_count as f64);
                
                Ok(EnforcementResult::AllPassed {
                    metrics,
                    fixes: response.applied_fixes,
                })
            }
            ProxyStatus::Modified => {
                let mut metrics = HashMap::new();
                metrics.insert("coverage".to_string(), response.metrics.coverage);
                
                Ok(EnforcementResult::PassedWithWarnings {
                    warnings: response.applied_fixes,
                    metrics,
                })
            }
            ProxyStatus::Rejected => {
                let failures: Vec<QualityFailure> = response.quality_report.violations
                    .into_iter()
                    .map(|v| QualityFailure {
                        gate: v.violation_type,
                        message: v.message,
                        severity: match v.severity {
                            crate::error::Severity::Error => FailureSeverity::Error,
                            crate::error::Severity::Warning => FailureSeverity::Warning,
                            crate::error::Severity::Info => FailureSeverity::Info,
                        },
                        file_path: Some(file_path.to_string()),
                        line_number: v.location.and_then(|loc| {
                            loc.split(':').nth(1).and_then(|s| s.parse().ok())
                        }),
                    })
                    .collect();
                
                Ok(EnforcementResult::Failed {
                    failures,
                    suggestions: response.quality_report.suggestions,
                })
            }
        }
    }
}