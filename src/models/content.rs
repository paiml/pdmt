//! Generated content models
//!
//! Data structures for representing generated content and associated metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generated content with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedContent {
    /// Unique identifier for this generated content
    #[cfg(feature = "todo-validation")]
    pub id: String,

    /// Template ID that generated this content
    pub template_id: String,

    /// The actual generated content
    pub content: String,

    /// Quality report if quality validation was performed
    #[cfg(feature = "quality-proxy")]
    pub quality_report: Option<crate::models::quality::QualityReport>,

    /// Generation timestamp
    #[cfg(feature = "todo-validation")]
    pub generated_at: chrono::DateTime<chrono::Utc>,

    /// Generation metadata
    pub metadata: GenerationMetadata,

    /// Input data that was used for generation
    pub input_data: serde_json::Value,
}

/// Metadata about the generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMetadata {
    /// Template version used
    pub template_version: String,

    /// Whether generation was deterministic
    pub is_deterministic: bool,

    /// Processing time in milliseconds
    pub processing_time_ms: u64,

    /// Number of validation passes
    pub validation_passes: usize,

    /// Whether refactoring was applied
    #[cfg(feature = "quality-proxy")]
    pub refactoring_applied: bool,

    /// Custom metadata fields
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Content format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentFormat {
    /// YAML format
    Yaml,
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
    /// Plain text
    Text,
}

/// Content validation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    /// Content passed all validations
    Passed,
    /// Content failed validation
    Failed,
    /// Content passed with warnings
    Warning,
    /// Validation was skipped
    Skipped,
    /// Validation is in progress
    Pending,
}

impl GeneratedContent {
    /// Create new generated content
    pub fn new(template_id: String, content: String, input_data: serde_json::Value) -> Self {
        Self {
            #[cfg(feature = "todo-validation")]
            id: crate::utils::generate_content_id(),
            template_id,
            content,
            #[cfg(feature = "quality-proxy")]
            quality_report: None,
            #[cfg(feature = "todo-validation")]
            generated_at: crate::utils::current_timestamp(),
            metadata: GenerationMetadata::default(),
            input_data,
        }
    }

    /// Get content as specified format
    pub fn as_format(&self, format: ContentFormat) -> crate::Result<String> {
        match format {
            ContentFormat::Yaml => Ok(self.content.clone()),
            ContentFormat::Json => {
                let value: serde_yaml::Value = serde_yaml::from_str(&self.content)?;
                let json = serde_json::to_string_pretty(&value)?;
                Ok(json)
            }
            ContentFormat::Markdown => {
                // Convert YAML content to markdown representation
                self.to_markdown()
            }
            ContentFormat::Text => {
                // Extract plain text from YAML content
                self.to_plain_text()
            }
        }
    }

    /// Convert content to markdown format
    fn to_markdown(&self) -> crate::Result<String> {
        let value: serde_yaml::Value = serde_yaml::from_str(&self.content)?;
        let mut markdown = String::new();

        if let Some(mapping) = value.as_mapping() {
            for (key, value) in mapping {
                if let Some(key_str) = key.as_str() {
                    markdown.push_str(&format!("## {}\n\n", key_str));
                    self.value_to_markdown(value, &mut markdown, 0)?;
                    markdown.push('\n');
                }
            }
        }

        Ok(markdown)
    }

    /// Convert YAML value to markdown recursively
    fn value_to_markdown(
        &self,
        value: &serde_yaml::Value,
        output: &mut String,
        indent: usize,
    ) -> crate::Result<()> {
        let indent_str = "  ".repeat(indent);

        match value {
            serde_yaml::Value::Sequence(seq) => {
                for item in seq {
                    if let Some(string_val) = item.as_str() {
                        output.push_str(&format!("{}* {}\n", indent_str, string_val));
                    } else if let Some(mapping) = item.as_mapping() {
                        for (key, val) in mapping {
                            if let Some(key_str) = key.as_str() {
                                output.push_str(&format!("{}* **{}**: ", indent_str, key_str));
                                if let Some(val_str) = val.as_str() {
                                    output.push_str(&format!("{}\n", val_str));
                                } else {
                                    output.push('\n');
                                    self.value_to_markdown(val, output, indent + 1)?;
                                }
                            }
                        }
                    }
                }
            }
            serde_yaml::Value::Mapping(mapping) => {
                for (key, val) in mapping {
                    if let Some(key_str) = key.as_str() {
                        output.push_str(&format!("{}**{}**: ", indent_str, key_str));
                        if let Some(val_str) = val.as_str() {
                            output.push_str(&format!("{}\n", val_str));
                        } else {
                            output.push('\n');
                            self.value_to_markdown(val, output, indent + 1)?;
                        }
                    }
                }
            }
            _ => {
                if let Some(string_val) = value.as_str() {
                    output.push_str(&format!("{}{}\n", indent_str, string_val));
                } else {
                    output.push_str(&format!("{}{:?}\n", indent_str, value));
                }
            }
        }

        Ok(())
    }

    /// Convert content to plain text
    fn to_plain_text(&self) -> crate::Result<String> {
        let value: serde_yaml::Value = serde_yaml::from_str(&self.content)?;
        let mut text = String::new();
        self.value_to_plain_text(&value, &mut text)?;
        Ok(text)
    }

    /// Convert YAML value to plain text recursively
    fn value_to_plain_text(
        &self,
        value: &serde_yaml::Value,
        output: &mut String,
    ) -> crate::Result<()> {
        match value {
            serde_yaml::Value::String(s) => {
                output.push_str(s);
                output.push(' ');
            }
            serde_yaml::Value::Sequence(seq) => {
                for item in seq {
                    self.value_to_plain_text(item, output)?;
                }
            }
            serde_yaml::Value::Mapping(mapping) => {
                for (_, val) in mapping {
                    self.value_to_plain_text(val, output)?;
                }
            }
            serde_yaml::Value::Number(n) => {
                output.push_str(&n.to_string());
                output.push(' ');
            }
            serde_yaml::Value::Bool(b) => {
                output.push_str(&b.to_string());
                output.push(' ');
            }
            serde_yaml::Value::Null => {}
            serde_yaml::Value::Tagged(_) => {} // Handle tagged values
        }
        Ok(())
    }

    /// Check if content has quality issues
    #[cfg(feature = "quality-proxy")]
    pub fn has_quality_issues(&self) -> bool {
        self.quality_report
            .as_ref()
            .map(|report| !report.passed)
            .unwrap_or(false)
    }

    /// Get processing time as duration
    pub fn processing_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.metadata.processing_time_ms)
    }
}

impl Default for GenerationMetadata {
    fn default() -> Self {
        Self {
            template_version: "unknown".to_string(),
            is_deterministic: true,
            processing_time_ms: 0,
            validation_passes: 0,
            #[cfg(feature = "quality-proxy")]
            refactoring_applied: false,
            custom_fields: HashMap::new(),
        }
    }
}

impl std::fmt::Display for ContentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentFormat::Yaml => write!(f, "yaml"),
            ContentFormat::Json => write!(f, "json"),
            ContentFormat::Markdown => write!(f, "markdown"),
            ContentFormat::Text => write!(f, "text"),
        }
    }
}

impl std::str::FromStr for ContentFormat {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(ContentFormat::Yaml),
            "json" => Ok(ContentFormat::Json),
            "markdown" | "md" => Ok(ContentFormat::Markdown),
            "text" | "txt" => Ok(ContentFormat::Text),
            _ => Err(crate::Error::invalid_input(format!(
                "Unknown format: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generated_content_creation() {
        let content = GeneratedContent::new(
            "test_template".to_string(),
            "test: content".to_string(),
            json!({"test": "input"}),
        );

        assert_eq!(content.template_id, "test_template");
        assert_eq!(content.content, "test: content");
        assert!(content.metadata.is_deterministic);
    }

    #[test]
    fn test_content_format_conversion() -> crate::Result<()> {
        let yaml_content = "todos:\n  - content: test task\n    status: pending";
        let content =
            GeneratedContent::new("test".to_string(), yaml_content.to_string(), json!({}));

        // Test YAML format (identity)
        let yaml_result = content.as_format(ContentFormat::Yaml)?;
        assert_eq!(yaml_result, yaml_content);

        // Test JSON format
        let json_result = content.as_format(ContentFormat::Json)?;
        assert!(json_result.contains("todos"));
        assert!(json_result.contains("test task"));

        // Test Markdown format
        let md_result = content.as_format(ContentFormat::Markdown)?;
        assert!(md_result.contains("## todos"));
        assert!(md_result.contains("* **content**: test task"));

        // Test plain text format
        let text_result = content.as_format(ContentFormat::Text)?;
        assert!(text_result.contains("test task"));
        assert!(text_result.contains("pending"));

        Ok(())
    }

    #[test]
    fn test_content_format_parsing() {
        assert_eq!(
            "yaml".parse::<ContentFormat>().unwrap(),
            ContentFormat::Yaml
        );
        assert_eq!(
            "json".parse::<ContentFormat>().unwrap(),
            ContentFormat::Json
        );
        assert_eq!(
            "markdown".parse::<ContentFormat>().unwrap(),
            ContentFormat::Markdown
        );
        assert_eq!(
            "text".parse::<ContentFormat>().unwrap(),
            ContentFormat::Text
        );

        assert!("invalid".parse::<ContentFormat>().is_err());
    }

    #[test]
    fn test_generation_metadata() {
        let mut metadata = GenerationMetadata::default();
        metadata.processing_time_ms = 150;
        metadata.validation_passes = 2;
        metadata
            .custom_fields
            .insert("test".to_string(), json!("value"));

        assert_eq!(metadata.processing_time_ms, 150);
        assert_eq!(metadata.validation_passes, 2);
        assert!(metadata.is_deterministic);
    }
}
