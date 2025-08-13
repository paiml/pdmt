//! Template definition structures
//!
//! Data structures for defining YAML templates with metadata and validation rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDefinition {
    /// Unique template identifier
    pub id: String,

    /// Template version (semantic versioning)
    pub version: String,

    /// Optional parent template to extend
    pub extends: Option<String>,

    /// Template metadata
    pub metadata: TemplateMetadata,

    /// Input schema definition
    pub input_schema: serde_json::Value,

    /// Output schema definition
    pub output_schema: OutputSchema,

    /// Validation rules
    pub validation: ValidationRules,

    /// Handlebars template string
    pub prompt_template: String,

    /// Quality enforcement configuration
    #[cfg(feature = "quality-proxy")]
    pub quality_enforcement: Option<QualityEnforcement>,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Provider type (deterministic, anthropic, etc.)
    pub provider: String,

    /// Human-readable description
    pub description: String,

    /// Provider-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Template author information
    pub author: Option<String>,

    /// Creation timestamp
    #[cfg(feature = "todo-validation")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Last modified timestamp
    #[cfg(feature = "todo-validation")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Template tags for categorization
    pub tags: Vec<String>,
}

/// Output schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSchema {
    /// Output format (yaml, json, text, markdown)
    pub format: String,

    /// Expected output structure description
    pub structure: String,

    /// JSON Schema for output validation
    pub schema: Option<serde_json::Value>,

    /// Example output
    pub example: Option<String>,
}

/// Validation rules for template processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Whether template must produce deterministic output
    pub deterministic_only: bool,

    /// Required fields in output
    pub required_fields: Vec<String>,

    /// Optional fields in output
    pub optional_fields: Vec<String>,

    /// Quality gate configurations
    pub quality_gates: Option<QualityGateRules>,

    /// Structure validation rules
    pub structure_rules: Option<StructureRules>,

    /// Custom validation functions
    pub custom_validators: Vec<String>,

    /// Minimum output length
    pub min_length: Option<usize>,

    /// Maximum output length
    pub max_length: Option<usize>,
}

/// Quality gate validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateRules {
    /// Maximum complexity per task/item
    pub max_complexity_per_task: Option<u8>,

    /// Whether time estimates are required
    pub require_time_estimates: bool,

    /// Whether specific actions are required
    pub require_specific_actions: bool,

    /// Minimum characters for task descriptions
    pub min_task_detail_chars: Option<usize>,

    /// Maximum characters for task descriptions
    pub max_task_detail_chars: Option<usize>,

    /// Custom quality rules
    pub custom_rules: HashMap<String, serde_json::Value>,
}

/// Structure validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureRules {
    /// Maximum number of items (todos, tasks, etc.)
    pub max_items: Option<usize>,

    /// Minimum number of items
    pub min_items: Option<usize>,

    /// Whether dependency graph is required
    pub require_dependency_graph: bool,

    /// Whether to prevent circular dependencies
    pub prevent_circular_dependencies: bool,

    /// Required structural elements
    pub required_elements: Vec<String>,

    /// Forbidden structural elements
    pub forbidden_elements: Vec<String>,
}

/// Quality enforcement configuration
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityEnforcement {
    /// PMAT configuration
    pub pmat_config: PmatConfig,

    /// Whether to auto-refactor on quality failures
    pub auto_refactor: bool,

    /// Quality enforcement mode
    pub mode: QualityMode,

    /// Custom quality thresholds
    pub thresholds: HashMap<String, f64>,
}

/// PMAT quality proxy configuration
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmatConfig {
    /// Enforcement mode (strict, advisory, auto_fix)
    pub mode: String,

    /// Maximum complexity threshold
    pub max_complexity: u32,

    /// Whether SATD (Self-Admitted Technical Debt) is allowed
    pub allow_satd: bool,

    /// Whether documentation is required
    pub require_docs: bool,

    /// Whether to auto-format code
    pub auto_format: bool,

    /// Custom PMAT settings
    pub custom_settings: HashMap<String, serde_json::Value>,
}

/// Quality enforcement modes
#[cfg(feature = "quality-proxy")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityMode {
    /// Strict mode - reject any quality failures
    Strict,
    /// Advisory mode - warn but allow quality issues
    Advisory,
    /// Auto-fix mode - automatically fix quality issues
    AutoFix,
    /// Disabled - skip quality enforcement
    Disabled,
}

impl TemplateDefinition {
    /// Create a new template definition
    pub fn new<S: Into<String>>(id: S, version: S, prompt_template: S) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            extends: None,
            metadata: TemplateMetadata::default(),
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
            output_schema: OutputSchema::default(),
            validation: ValidationRules::default(),
            prompt_template: prompt_template.into(),
            #[cfg(feature = "quality-proxy")]
            quality_enforcement: None,
        }
    }

    /// Check if template is deterministic
    pub fn is_deterministic(&self) -> bool {
        // Provider is deterministic
        if self.metadata.provider == "deterministic" {
            return true;
        }

        // Or temperature is 0.0
        self.metadata
            .parameters
            .get("temperature")
            .and_then(|v| v.as_f64())
            .map(|t| t == 0.0)
            .unwrap_or(false)
    }

    /// Get template parameter value
    pub fn get_parameter<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.metadata
            .parameters
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Set template parameter
    pub fn set_parameter<T>(&mut self, key: String, value: T) -> crate::Result<()>
    where
        T: Serialize,
    {
        let json_value = serde_json::to_value(value)?;
        self.metadata.parameters.insert(key, json_value);
        Ok(())
    }

    /// Validate template definition
    pub fn validate(&self) -> crate::Result<()> {
        // Check required fields
        if self.id.is_empty() {
            return Err(crate::error::TemplateError::InvalidDefinition {
                reason: "Template ID cannot be empty".to_string(),
            }
            .into());
        }

        if self.version.is_empty() {
            return Err(crate::error::TemplateError::InvalidDefinition {
                reason: "Template version cannot be empty".to_string(),
            }
            .into());
        }

        if self.prompt_template.is_empty() {
            return Err(crate::error::TemplateError::InvalidDefinition {
                reason: "Prompt template cannot be empty".to_string(),
            }
            .into());
        }

        // Validate input schema is valid JSON
        if !self.input_schema.is_object() {
            return Err(crate::error::TemplateError::InvalidDefinition {
                reason: "Input schema must be a JSON object".to_string(),
            }
            .into());
        }

        // Validate deterministic settings
        if self.validation.deterministic_only && !self.is_deterministic() {
            return Err(crate::error::TemplateError::InvalidDefinition {
                reason: "Template marked as deterministic_only but provider/parameters are non-deterministic".to_string(),
            }.into());
        }

        Ok(())
    }

    /// Get all template tags (including inherited ones)
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags = self.metadata.tags.clone();

        // Add automatic tags based on template properties
        if self.is_deterministic() {
            tags.push("deterministic".to_string());
        }

        if self.validation.deterministic_only {
            tags.push("strict".to_string());
        }

        #[cfg(feature = "quality-proxy")]
        if self.quality_enforcement.is_some() {
            tags.push("quality-enforced".to_string());
        }

        // Remove duplicates
        tags.sort();
        tags.dedup();

        tags
    }
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        Self {
            provider: "deterministic".to_string(),
            description: "Template generated by PDMT".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("temperature".to_string(), serde_json::json!(0.0));
                params
            },
            author: None,
            #[cfg(feature = "todo-validation")]
            created_at: Some(chrono::Utc::now()),
            #[cfg(feature = "todo-validation")]
            modified_at: Some(chrono::Utc::now()),
            tags: Vec::new(),
        }
    }
}

impl Default for OutputSchema {
    fn default() -> Self {
        Self {
            format: "yaml".to_string(),
            structure: "Generated content structure".to_string(),
            schema: None,
            example: None,
        }
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            deterministic_only: true,
            required_fields: Vec::new(),
            optional_fields: Vec::new(),
            quality_gates: None,
            structure_rules: None,
            custom_validators: Vec::new(),
            min_length: Some(10),
            max_length: Some(10000),
        }
    }
}

impl Default for QualityGateRules {
    fn default() -> Self {
        Self {
            max_complexity_per_task: Some(8),
            require_time_estimates: true,
            require_specific_actions: true,
            min_task_detail_chars: Some(10),
            max_task_detail_chars: Some(100),
            custom_rules: HashMap::new(),
        }
    }
}

impl Default for StructureRules {
    fn default() -> Self {
        Self {
            max_items: Some(50),
            min_items: Some(1),
            require_dependency_graph: true,
            prevent_circular_dependencies: true,
            required_elements: Vec::new(),
            forbidden_elements: Vec::new(),
        }
    }
}

#[cfg(feature = "quality-proxy")]
impl Default for QualityEnforcement {
    fn default() -> Self {
        Self {
            pmat_config: PmatConfig::default(),
            auto_refactor: false,
            mode: QualityMode::Strict,
            thresholds: HashMap::new(),
        }
    }
}

#[cfg(feature = "quality-proxy")]
impl Default for PmatConfig {
    fn default() -> Self {
        Self {
            mode: "strict".to_string(),
            max_complexity: 8,
            allow_satd: false,
            require_docs: true,
            auto_format: true,
            custom_settings: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_definition_creation() {
        let template =
            TemplateDefinition::new("test_template", "1.0.0", "Generate content for {{input}}");

        assert_eq!(template.id, "test_template");
        assert_eq!(template.version, "1.0.0");
        assert!(template.is_deterministic());
    }

    #[test]
    fn test_template_validation() {
        let mut template = TemplateDefinition::new("test", "1.0", "{{input}}");
        assert!(template.validate().is_ok());

        // Test empty ID
        template.id = String::new();
        assert!(template.validate().is_err());
    }

    #[test]
    fn test_deterministic_detection() {
        let mut template = TemplateDefinition::new("test", "1.0", "{{input}}");
        assert!(template.is_deterministic());

        // Change to non-deterministic provider and temperature
        template.metadata.provider = "anthropic".to_string();
        template
            .set_parameter("temperature".to_string(), 0.7)
            .unwrap();
        assert!(!template.is_deterministic());

        // Set deterministic temperature with non-deterministic provider
        template
            .set_parameter("temperature".to_string(), 0.0)
            .unwrap();
        assert!(template.is_deterministic());

        // Change back to deterministic provider
        template.metadata.provider = "deterministic".to_string();
        assert!(template.is_deterministic());
    }

    #[test]
    fn test_parameter_management() {
        let mut template = TemplateDefinition::new("test", "1.0", "{{input}}");

        // Set and get parameter
        template
            .set_parameter("max_tokens".to_string(), 100)
            .unwrap();
        let max_tokens: Option<i32> = template.get_parameter("max_tokens");
        assert_eq!(max_tokens, Some(100));

        // Non-existent parameter
        let missing: Option<String> = template.get_parameter("missing");
        assert_eq!(missing, None);
    }

    #[test]
    fn test_template_tags() {
        let mut template = TemplateDefinition::new("test", "1.0", "{{input}}");
        template.metadata.tags.push("test".to_string());
        template.validation.deterministic_only = true;

        let all_tags = template.get_all_tags();
        assert!(all_tags.contains(&"test".to_string()));
        assert!(all_tags.contains(&"deterministic".to_string()));
        assert!(all_tags.contains(&"strict".to_string()));
    }
}
