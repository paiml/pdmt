//! # PDMT - Pragmatic Deterministic MCP Templating
//!
//! A high-performance, deterministic templating library for Model Context Protocol (MCP)
//! applications, designed for generating consistent, validated todo lists and structured
//! content with quality enforcement.
//!
//! ## Features
//!
//! - **🎯 Deterministic Generation**: 0.0 temperature templating ensures reproducible outputs
//! - **📋 Todo Validation**: Comprehensive validation with actionability checks, complexity scoring, and time estimates
//! - **🔄 MCP Integration**: Native support for Model Context Protocol via PMCP SDK
//! - **🛡️ Quality Gates**: PMAT (Paiml MCP Agent Toolkit) integration for quality enforcement
//! - **🧪 Extensive Testing**: 81%+ test coverage with property testing, fuzz testing, and edge case coverage
//! - **⚡ High Performance**: Optimized Handlebars engine with caching and validation
//! - **📦 Multiple Formats**: Support for YAML, JSON, Markdown, and plain text output
//! - **🔍 Dependency Analysis**: Circular dependency detection and critical path calculation
//!
//! ## Quick Start
//!
//! ```rust
//! use pdmt::{TemplateEngine, models::todo::TodoInput};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create template engine
//!     let mut engine = TemplateEngine::new();
//!     
//!     // Load built-in todo list template
//!     engine.load_builtin_templates().await?;
//!     
//!     // Generate deterministic todo list
//!     let input = json!({
//!         "project_name": "Rust Web API",
//!         "requirements": [
//!             "Create user authentication system",
//!             "Implement REST API endpoints",
//!             "Add database integration"
//!         ],
//!         "granularity": "high"
//!     });
//!     
//!     let result = engine.generate("todo_list", input).await?;
//!     println!("{}", serde_yaml::to_string(&result.content)?);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several core modules:
//!
//! - [`template`]: Template engine and YAML template processing
//! - [`quality`]: Quality enforcement and PMAT integration  
//! - [`mcp`]: Model Context Protocol tools and handlers
//! - [`validators`]: Content validation including todo lists
//! - [`models`]: Data structures for templates and generated content
//!
//! ## Template System
//!
//! Templates are defined in YAML format with rich metadata:
//!
//! ```yaml
//! id: todo_list
//! version: "1.0.0"
//! extends: base
//!
//! metadata:
//!   provider: "deterministic"
//!   description: "Generate deterministic todo lists"
//!   parameters:
//!     temperature: 0.0  # Ensures deterministic output
//!     
//! input_schema:
//!   type: object
//!   required: ["project_name", "requirements"]
//!   # ... schema definition
//!   
//! validation:
//!   deterministic_only: true
//!   quality_gates:
//!     max_complexity_per_task: 8
//!     require_time_estimates: true
//!     # ... quality rules
//! ```
//!
//! ## Quality Enforcement
//!
//! Every generated template can be validated through PMAT quality gates:
//!
//! - **Complexity Analysis**: Prevent overly complex task descriptions
//! - **SATD Detection**: Zero tolerance for TODO/FIXME comments in output
//! - **Actionability Check**: Ensure all todos are specific and actionable
//! - **Dependency Validation**: Prevent circular dependencies in task graphs
//!
//! ## MCP Integration
//!
//! The library provides native MCP tools for use with AI coding assistants:
//!
//! ```rust,ignore
//! use pdmt::mcp::create_template_tool;
//!
//! // Register with MCP server
//! let template_tool = create_template_tool();
//! mcp_server.register_tool(template_tool);
//! ```

#![doc(html_root_url = "https://docs.rs/pdmt/")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

// Core modules
pub mod error;
pub mod models;
pub mod template;
pub mod validators;

// Optional feature modules
#[cfg(feature = "quality-proxy")]
#[cfg_attr(docsrs, doc(cfg(feature = "quality-proxy")))]
pub mod quality;

#[cfg(feature = "mcp-tools")]
#[cfg_attr(docsrs, doc(cfg(feature = "mcp-tools")))]
pub mod mcp;

// Re-exports for convenience
pub use crate::error::{Error, Result};
pub use crate::models::content::GeneratedContent;
pub use crate::template::definition::TemplateDefinition;
pub use crate::template::engine::TemplateEngine;

#[cfg(feature = "todo-validation")]
pub use crate::validators::todo::TodoValidator;

#[cfg(feature = "mcp-tools")]
pub use crate::mcp::tools::create_template_tool;

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default template directory for builtin templates
pub const DEFAULT_TEMPLATE_DIR: &str = "templates";

/// Maximum file size for template processing (10MB)
pub const MAX_TEMPLATE_SIZE: usize = 10 * 1024 * 1024;

/// Default timeout for quality validation (30 seconds)
#[cfg(feature = "quality-proxy")]
pub const DEFAULT_QUALITY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Built-in template identifiers
pub mod builtin {
    /// Todo list generation template
    pub const TODO_LIST: &str = "todo_list";

    /// Project scaffold template  
    pub const PROJECT_SCAFFOLD: &str = "project_scaffold";

    /// Base template (extended by others)
    pub const BASE: &str = "base";
}

/// Utility functions for common operations
pub mod utils {
    use crate::error::{Error, Result};

    /// Validate that a template ID is valid
    ///
    /// # Examples
    ///
    /// ```
    /// use pdmt::utils::validate_template_id;
    ///
    /// assert!(validate_template_id("todo_list").is_ok());
    /// assert!(validate_template_id("invalid-id!").is_err());
    /// ```
    pub fn validate_template_id(id: &str) -> Result<()> {
        if id.is_empty() {
            return Err(Error::InvalidInput("Template ID cannot be empty".into()));
        }

        if id.len() > 64 {
            return Err(Error::InvalidInput(
                "Template ID too long (max 64 chars)".into(),
            ));
        }

        if !id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(Error::InvalidInput(
                "Template ID contains invalid characters".into(),
            ));
        }

        Ok(())
    }

    /// Generate a unique content ID
    #[cfg(feature = "todo-validation")]
    pub fn generate_content_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Get current timestamp for content generation
    #[cfg(feature = "todo-validation")]
    pub fn current_timestamp() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version_defined() {
        // VERSION comes from env!("CARGO_PKG_VERSION") which is always non-empty at build time
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_validate_template_id() {
        // Valid IDs
        assert!(utils::validate_template_id("todo_list").is_ok());
        assert!(utils::validate_template_id("test123").is_ok());
        assert!(utils::validate_template_id("a").is_ok());

        // Invalid IDs
        assert!(utils::validate_template_id("").is_err());
        assert!(utils::validate_template_id("invalid-id").is_err());
        assert!(utils::validate_template_id("invalid!id").is_err());
        assert!(utils::validate_template_id(&"x".repeat(65)).is_err());
    }

    #[cfg(feature = "todo-validation")]
    #[test]
    fn test_content_id_generation() {
        let id1 = utils::generate_content_id();
        let id2 = utils::generate_content_id();
        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
    }
}
