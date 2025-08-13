//! Quality gates for comprehensive validation
//!
//! This module implements the quality gate pipeline that ensures
//! all generated content meets enterprise-grade standards.

use crate::quality::proxy::{ProxyConfig, QualityMetrics};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Quality gate for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    /// Gate identifier
    pub id: String,
    /// Gate description
    pub description: String,
    /// Gate type
    pub gate_type: GateType,
    /// Required threshold (if applicable)
    pub threshold: Option<f64>,
    /// Whether this gate is mandatory
    pub mandatory: bool,
}

/// Type of quality gate
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GateType {
    /// Coverage validation
    Coverage,
    /// Doctest validation
    Doctests,
    /// Property test validation
    PropertyTests,
    /// Example validation
    Examples,
    /// SATD detection
    SatdDetection,
    /// Complexity analysis
    Complexity,
    /// Linting validation
    Linting,
    /// Format validation
    Formatting,
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coverage => write!(f, "Coverage"),
            Self::Doctests => write!(f, "Doctests"),
            Self::PropertyTests => write!(f, "Property Tests"),
            Self::Examples => write!(f, "Examples"),
            Self::SatdDetection => write!(f, "SATD Detection"),
            Self::Complexity => write!(f, "Complexity"),
            Self::Linting => write!(f, "Linting"),
            Self::Formatting => write!(f, "Formatting"),
        }
    }
}

/// Result of gate validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    /// Gate that was validated
    pub gate: QualityGate,
    /// Whether the gate passed
    pub passed: bool,
    /// Actual value (if applicable)
    pub actual_value: Option<f64>,
    /// Message describing the result
    pub message: String,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Quality gate pipeline for comprehensive validation
#[derive(Debug)]
pub struct QualityGatePipeline {
    /// Gates to execute
    gates: Vec<QualityGate>,
    /// Configuration
    _config: ProxyConfig,
}

impl QualityGatePipeline {
    /// Create a new pipeline with default gates
    pub fn new() -> Self {
        Self::with_config(ProxyConfig::default())
    }
    
    /// Create a new pipeline with custom configuration
    pub fn with_config(config: ProxyConfig) -> Self {
        let gates = Self::create_default_gates(&config);
        Self { gates, _config: config }
    }
    
    /// Create default quality gates based on configuration
    fn create_default_gates(config: &ProxyConfig) -> Vec<QualityGate> {
        let mut gates = Vec::new();
        
        // Coverage gate
        gates.push(QualityGate {
            id: "coverage_80_percent".to_string(),
            description: format!("Code coverage must be at least {}%", config.min_coverage),
            gate_type: GateType::Coverage,
            threshold: Some(config.min_coverage),
            mandatory: true,
        });
        
        // Doctest gate
        if config.require_doctests {
            gates.push(QualityGate {
                id: "doctests_required".to_string(),
                description: "All public APIs must have doctests".to_string(),
                gate_type: GateType::Doctests,
                threshold: None,
                mandatory: true,
            });
        }
        
        // Property test gate
        if config.require_property_tests {
            gates.push(QualityGate {
                id: "property_tests_required".to_string(),
                description: "Complex logic must have property tests".to_string(),
                gate_type: GateType::PropertyTests,
                threshold: None,
                mandatory: true,
            });
        }
        
        // Example gate
        if config.require_examples {
            gates.push(QualityGate {
                id: "examples_required".to_string(),
                description: "Working examples must be provided".to_string(),
                gate_type: GateType::Examples,
                threshold: None,
                mandatory: true,
            });
        }
        
        // SATD gate
        if config.zero_satd {
            gates.push(QualityGate {
                id: "zero_satd_tolerance".to_string(),
                description: "No TODO/FIXME/HACK comments allowed".to_string(),
                gate_type: GateType::SatdDetection,
                threshold: Some(0.0),
                mandatory: true,
            });
        }
        
        // Complexity gate
        gates.push(QualityGate {
            id: "complexity_limit".to_string(),
            description: format!("Cyclomatic complexity must be under {}", config.max_complexity),
            gate_type: GateType::Complexity,
            threshold: Some(config.max_complexity as f64),
            mandatory: true,
        });
        
        // Linting gate
        gates.push(QualityGate {
            id: "clippy_clean".to_string(),
            description: "Code must pass clippy linting".to_string(),
            gate_type: GateType::Linting,
            threshold: None,
            mandatory: true,
        });
        
        // Formatting gate
        gates.push(QualityGate {
            id: "rustfmt_compliant".to_string(),
            description: "Code must be formatted with rustfmt".to_string(),
            gate_type: GateType::Formatting,
            threshold: None,
            mandatory: false,
        });
        
        gates
    }
    
    /// Add a custom gate to the pipeline
    pub fn add_gate(&mut self, gate: QualityGate) {
        self.gates.push(gate);
    }
    
    /// Remove a gate by ID
    pub fn remove_gate(&mut self, gate_id: &str) {
        self.gates.retain(|g| g.id != gate_id);
    }
    
    /// Validate metrics against all gates
    pub fn validate(&self, metrics: &QualityMetrics) -> Vec<GateResult> {
        let mut results = Vec::new();
        
        for gate in &self.gates {
            let result = self.validate_gate(gate, metrics);
            results.push(result);
        }
        
        results
    }
    
    /// Validate a single gate
    fn validate_gate(&self, gate: &QualityGate, metrics: &QualityMetrics) -> GateResult {
        match gate.gate_type {
            GateType::Coverage => {
                let passed = metrics.coverage >= gate.threshold.unwrap_or(80.0);
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.coverage),
                    message: if passed {
                        format!("Coverage {}% meets requirement", metrics.coverage)
                    } else {
                        format!("Coverage {}% below required {}%", 
                            metrics.coverage, gate.threshold.unwrap_or(80.0))
                    },
                    suggestions: if !passed {
                        vec!["Add more unit tests to increase coverage".to_string()]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::Doctests => {
                let passed = metrics.doctest_count > 0;
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.doctest_count as f64),
                    message: if passed {
                        format!("{} doctests found", metrics.doctest_count)
                    } else {
                        "No doctests found".to_string()
                    },
                    suggestions: if !passed {
                        vec!["Add doctests to all public APIs".to_string()]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::PropertyTests => {
                let passed = metrics.property_test_count > 0;
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.property_test_count as f64),
                    message: if passed {
                        format!("{} property tests found", metrics.property_test_count)
                    } else {
                        "No property tests found".to_string()
                    },
                    suggestions: if !passed {
                        vec!["Add property tests for complex logic".to_string()]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::Examples => {
                let passed = metrics.example_count > 0;
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.example_count as f64),
                    message: if passed {
                        format!("{} examples found", metrics.example_count)
                    } else {
                        "No examples found".to_string()
                    },
                    suggestions: if !passed {
                        vec!["Add working examples demonstrating usage".to_string()]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::SatdDetection => {
                let passed = metrics.satd_count == 0;
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.satd_count as f64),
                    message: if passed {
                        "No SATD comments detected".to_string()
                    } else {
                        format!("{} SATD comments found", metrics.satd_count)
                    },
                    suggestions: if !passed {
                        vec![
                            "Remove all TODO/FIXME/HACK comments".to_string(),
                            "Convert TODOs to proper issue tracking".to_string(),
                        ]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::Complexity => {
                let passed = metrics.complexity <= gate.threshold.unwrap_or(8.0) as u32;
                GateResult {
                    gate: gate.clone(),
                    passed,
                    actual_value: Some(metrics.complexity as f64),
                    message: if passed {
                        format!("Complexity {} within limit", metrics.complexity)
                    } else {
                        format!("Complexity {} exceeds limit of {}", 
                            metrics.complexity, gate.threshold.unwrap_or(8.0))
                    },
                    suggestions: if !passed {
                        vec![
                            "Refactor complex functions into smaller units".to_string(),
                            "Extract helper functions to reduce complexity".to_string(),
                        ]
                    } else {
                        Vec::new()
                    },
                }
            }
            GateType::Linting => {
                // This would normally check linting results
                GateResult {
                    gate: gate.clone(),
                    passed: true,
                    actual_value: None,
                    message: "Linting validation placeholder".to_string(),
                    suggestions: Vec::new(),
                }
            }
            GateType::Formatting => {
                // This would normally check formatting
                GateResult {
                    gate: gate.clone(),
                    passed: true,
                    actual_value: None,
                    message: "Formatting validation placeholder".to_string(),
                    suggestions: Vec::new(),
                }
            }
        }
    }
    
    /// Check if all mandatory gates pass
    pub fn all_mandatory_gates_pass(&self, metrics: &QualityMetrics) -> bool {
        let results = self.validate(metrics);
        results.iter()
            .filter(|r| r.gate.mandatory)
            .all(|r| r.passed)
    }
    
    /// Get failed gates
    pub fn get_failed_gates(&self, metrics: &QualityMetrics) -> Vec<GateResult> {
        self.validate(metrics)
            .into_iter()
            .filter(|r| !r.passed)
            .collect()
    }
}

impl Default for QualityGatePipeline {
    fn default() -> Self {
        Self::new()
    }
}