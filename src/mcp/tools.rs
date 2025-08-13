//! MCP tool definitions

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP tool information for PDMT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name identifier
    pub name: String,
    /// Human-readable description of the tool
    pub description: String,
    /// JSON schema for tool input validation
    pub input_schema: Value,
}

/// Create the deterministic template MCP tool definition
pub fn create_template_tool() -> ToolDefinition {
    ToolDefinition {
        name: "deterministic_template".to_string(),
        description: "Generate deterministic content using PDMT templates".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "template_id": {
                    "type": "string",
                    "description": "ID of the template to use"
                },
                "input": {
                    "type": "object",
                    "description": "Input data for template generation"
                },
                "quality_mode": {
                    "type": "string",
                    "enum": ["strict", "advisory", "auto_fix"],
                    "default": "strict",
                    "description": "Quality enforcement mode"
                },
                "output_format": {
                    "type": "string",
                    "enum": ["yaml", "json", "markdown"],
                    "default": "yaml",
                    "description": "Output format"
                }
            },
            "required": ["template_id", "input"],
            "additionalProperties": false
        }),
    }
}
