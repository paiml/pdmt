//! Template engine implementation
//!
//! Core engine for processing templates and generating content.

use crate::error::{Result, TemplateError};
use crate::models::content::GeneratedContent;
use crate::template::definition::TemplateDefinition;
use handlebars::Handlebars;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Main template engine
#[derive(Debug)]
pub struct TemplateEngine {
    /// Loaded template definitions
    templates: HashMap<String, TemplateDefinition>,

    /// Handlebars renderer
    handlebars: Handlebars<'static>,

    /// Quality proxy integration
    #[cfg(feature = "quality-proxy")]
    quality_proxy: Option<Arc<crate::quality::QualityProxy>>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Register helper functions
        handlebars.register_helper("upper", Box::new(uppercase_helper));
        handlebars.register_helper("lower", Box::new(lowercase_helper));
        handlebars.register_helper("capitalize", Box::new(capitalize_helper));

        Self {
            templates: HashMap::new(),
            handlebars,
            #[cfg(feature = "quality-proxy")]
            quality_proxy: None,
        }
    }

    /// Load builtin templates
    pub async fn load_builtin_templates(&mut self) -> Result<()> {
        // Load the todo list template
        let todo_template = create_todo_list_template();
        self.register_template(todo_template)?;

        // Load base template
        let base_template = create_base_template();
        self.register_template(base_template)?;

        info!("Loaded {} builtin templates", self.templates.len());
        Ok(())
    }

    /// Register a template definition
    pub fn register_template(&mut self, template: TemplateDefinition) -> Result<()> {
        template.validate()?;

        // Register with handlebars
        self.handlebars
            .register_template_string(&template.id, &template.prompt_template)
            .map_err(TemplateError::from)?;

        info!(
            "Registered template: {} (v{})",
            template.id, template.version
        );
        self.templates.insert(template.id.clone(), template);

        Ok(())
    }

    /// Generate content using a template
    pub async fn generate<T>(&self, template_id: &str, input: T) -> Result<GeneratedContent>
    where
        T: Serialize,
    {
        let start_time = std::time::Instant::now();

        debug!("Generating content with template: {}", template_id);

        // Get template definition
        let template = self
            .templates
            .get(template_id)
            .ok_or_else(|| TemplateError::not_found(template_id))?;

        // Serialize input to JSON value for storage
        let input_json = serde_json::to_value(&input)?;

        // Render template
        let rendered_content = self
            .handlebars
            .render(&template.id, &input)
            .map_err(TemplateError::from)?;

        // Create generated content
        let mut generated =
            GeneratedContent::new(template_id.to_string(), rendered_content, input_json);

        // Update metadata
        generated
            .metadata
            .template_version
            .clone_from(&template.version);
        generated.metadata.is_deterministic = template.is_deterministic();
        generated.metadata.processing_time_ms =
            start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;

        info!(
            "Generated content for template {} in {:?}",
            template_id,
            start_time.elapsed()
        );

        Ok(generated)
    }

    /// Get list of available templates
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }

    /// Get template definition by ID
    pub fn get_template(&self, template_id: &str) -> Option<&TemplateDefinition> {
        self.templates.get(template_id)
    }

    /// Enable quality proxy integration
    #[cfg(feature = "quality-proxy")]
    pub fn enable_quality_proxy(&mut self, proxy: Arc<crate::quality::QualityProxy>) {
        self.quality_proxy = Some(proxy);
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Create the built-in todo list template
fn create_todo_list_template() -> TemplateDefinition {
    use crate::template::definition::{
        OutputSchema, QualityGateRules, StructureRules, TemplateMetadata, ValidationRules,
    };

    let mut template = TemplateDefinition::new(
        "todo_list",
        "1.0.0",
        r#"# Todo List Template
todos:
{{#each requirements}}
  - id: "todo_{{@index}}"
    content: "Implement {{this}}"
    status: "pending"
    priority: "medium"
    estimated_hours: 4.0
    dependencies: []
    tags: ["implementation"]
{{/each}}"#,
    );

    template.metadata = TemplateMetadata {
        provider: "deterministic".to_string(),
        description: "Generate deterministic todo lists with quality enforcement".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("temperature".to_string(), serde_json::json!(0.0));
            params
        },
        author: Some("PDMT Team".to_string()),
        #[cfg(feature = "todo-validation")]
        created_at: Some(chrono::Utc::now()),
        #[cfg(feature = "todo-validation")]
        modified_at: Some(chrono::Utc::now()),
        tags: vec!["todo".to_string(), "deterministic".to_string()],
    };

    template.input_schema = serde_json::json!({
        "type": "object",
        "required": ["project_name", "requirements"],
        "properties": {
            "project_name": {
                "type": "string",
                "description": "Name of the project"
            },
            "requirements": {
                "type": "array",
                "items": {"type": "string"},
                "description": "List of requirements to convert to tasks"
            },
            "granularity": {
                "type": "string",
                "enum": ["high", "medium", "low"],
                "default": "high",
                "description": "Level of task detail"
            }
        }
    });

    template.output_schema = OutputSchema {
        format: "yaml".to_string(),
        structure: "todos: array of todo objects with id, content, status, priority, etc."
            .to_string(),
        schema: Some(serde_json::json!({
            "type": "object",
            "required": ["todos"],
            "properties": {
                "todos": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "content", "status", "priority"],
                        "properties": {
                            "id": {"type": "string"},
                            "content": {"type": "string", "minLength": 10, "maxLength": 100},
                            "status": {"type": "string", "enum": ["pending", "in_progress", "completed"]},
                            "priority": {"type": "string", "enum": ["low", "medium", "high", "critical"]},
                            "estimated_hours": {"type": "number", "minimum": 0.5, "maximum": 40},
                            "dependencies": {"type": "array", "items": {"type": "string"}},
                            "tags": {"type": "array", "items": {"type": "string"}}
                        }
                    }
                }
            }
        })),
        example: Some(
            r#"todos:
  - id: "todo_0"
    content: "Implement user authentication"
    status: "pending"
    priority: "high"
    estimated_hours: 4.0"#
                .to_string(),
        ),
    };

    template.validation = ValidationRules {
        deterministic_only: true,
        required_fields: vec!["todos".to_string()],
        optional_fields: vec!["metadata".to_string(), "project".to_string()],
        quality_gates: Some(QualityGateRules {
            max_complexity_per_task: Some(8),
            require_time_estimates: true,
            require_specific_actions: true,
            min_task_detail_chars: Some(10),
            max_task_detail_chars: Some(100),
            custom_rules: HashMap::new(),
        }),
        structure_rules: Some(StructureRules {
            max_items: Some(50),
            min_items: Some(1),
            require_dependency_graph: true,
            prevent_circular_dependencies: true,
            required_elements: vec!["todos".to_string()],
            forbidden_elements: Vec::new(),
        }),
        custom_validators: vec!["todo_validator".to_string()],
        min_length: Some(10),
        max_length: Some(50000),
    };

    template
}

/// Create the base template
fn create_base_template() -> TemplateDefinition {
    let template = TemplateDefinition::new(
        "base",
        "1.0.0",
        "# Base template for inheritance\n{{> content}}",
    );

    template
}

// Handlebars helper functions

fn uppercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&param.to_uppercase())?;
    Ok(())
}

fn lowercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn capitalize_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");

    if let Some(first) = param.chars().next() {
        let capitalized = first.to_uppercase().collect::<String>() + &param[first.len_utf8()..];
        out.write(&capitalized)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_template_engine_creation() {
        let engine = TemplateEngine::new();
        assert_eq!(engine.templates.len(), 0);
    }

    #[tokio::test]
    async fn test_builtin_template_loading() {
        let mut engine = TemplateEngine::new();
        engine.load_builtin_templates().await.unwrap();

        assert!(engine.templates.contains_key("todo_list"));
        assert!(engine.templates.contains_key("base"));
    }

    #[tokio::test]
    async fn test_simple_template_generation() {
        let mut engine = TemplateEngine::new();

        // Register a simple test template
        let template = TemplateDefinition::new("test", "1.0.0", "Hello {{name}}!");
        engine.register_template(template).unwrap();

        // Generate content
        let input = json!({"name": "World"});
        let result = engine.generate("test", input).await.unwrap();

        assert_eq!(result.content, "Hello World!");
        assert_eq!(result.template_id, "test");
    }

    #[test]
    fn test_handlebars_helpers() {
        let mut hb = Handlebars::new();
        hb.register_helper("upper", Box::new(uppercase_helper));
        hb.register_helper("lower", Box::new(lowercase_helper));
        hb.register_helper("capitalize", Box::new(capitalize_helper));

        hb.register_template_string("test", "{{upper name}} {{lower name}} {{capitalize name}}")
            .unwrap();

        let data = json!({"name": "hello"});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "HELLO hello Hello");
    }
}
