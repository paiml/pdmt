use pdmt::models::todo::TodoInput;
use pdmt::{TemplateDefinition, TemplateEngine};
use serde_json::json;

#[tokio::test]
async fn test_template_engine_full_workflow() {
    let mut engine = TemplateEngine::new();

    // Load builtin templates
    engine.load_builtin_templates().await.unwrap();

    // Create todo input
    let input = TodoInput {
        project_name: "Test Project".to_string(),
        requirements: vec![
            "Implement feature A".to_string(),
            "Create API endpoint".to_string(),
            "Add documentation".to_string(),
        ],
        granularity: pdmt::models::todo::TodoGranularity::High,
        project_context: None,
        quality_config: None,
        max_todos: Some(10),
        include_estimates: true,
        default_priority: Some(pdmt::models::todo::TodoPriority::High),
    };

    // Generate content
    let result = engine.generate("todo_list", input).await.unwrap();
    assert!(result.content.contains("todo_0"));
    assert!(result.content.contains("Implement"));
    assert_eq!(result.template_id, "todo_list");
}

#[tokio::test]
async fn test_custom_template_registration() {
    let mut engine = TemplateEngine::new();

    // Create custom template
    let template = TemplateDefinition::new(
        "custom_test",
        "1.0.0",
        "Hello {{name}}, your score is {{score}}!",
    );

    engine.register_template(template).unwrap();

    // Generate with custom template
    let input = json!({
        "name": "Alice",
        "score": 95
    });

    let result = engine.generate("custom_test", input).await.unwrap();
    assert!(result.content.contains("Hello Alice"));
    assert!(result.content.contains("95"));
}

#[test]
fn test_template_definition_validation() {
    // Valid template
    let valid = TemplateDefinition::new("test", "1.0.0", "{{content}}");
    assert!(valid.validate().is_ok());

    // Invalid - empty ID
    let mut invalid = TemplateDefinition::new("", "1.0.0", "{{content}}");
    assert!(invalid.validate().is_err());

    // Invalid - empty version
    invalid = TemplateDefinition::new("test", "", "{{content}}");
    assert!(invalid.validate().is_err());

    // Invalid - empty prompt
    invalid = TemplateDefinition::new("test", "1.0.0", "");
    assert!(invalid.validate().is_err());
}

#[test]
fn test_template_parameter_management() {
    let mut template = TemplateDefinition::new("test", "1.0.0", "{{test}}");

    // Set various parameter types
    template
        .set_parameter("string_param".to_string(), "value")
        .unwrap();
    template
        .set_parameter("number_param".to_string(), 42)
        .unwrap();
    template
        .set_parameter("bool_param".to_string(), true)
        .unwrap();
    template
        .set_parameter("array_param".to_string(), vec![1, 2, 3])
        .unwrap();

    // Get parameters with correct types
    let string_val: Option<String> = template.get_parameter("string_param");
    assert_eq!(string_val, Some("value".to_string()));

    let number_val: Option<i32> = template.get_parameter("number_param");
    assert_eq!(number_val, Some(42));

    let bool_val: Option<bool> = template.get_parameter("bool_param");
    assert_eq!(bool_val, Some(true));

    let array_val: Option<Vec<i32>> = template.get_parameter("array_param");
    assert_eq!(array_val, Some(vec![1, 2, 3]));

    // Non-existent parameter
    let missing: Option<String> = template.get_parameter("missing");
    assert!(missing.is_none());
}

#[test]
fn test_template_determinism_checks() {
    let mut template = TemplateDefinition::new("test", "1.0.0", "{{test}}");

    // Default is deterministic
    assert!(template.is_deterministic());

    // Change provider to non-deterministic
    template.metadata.provider = "openai".to_string();
    template
        .set_parameter("temperature".to_string(), 0.8)
        .unwrap();
    assert!(!template.is_deterministic());

    // Set temperature to 0
    template
        .set_parameter("temperature".to_string(), 0.0)
        .unwrap();
    assert!(template.is_deterministic());

    // Deterministic provider always deterministic
    template.metadata.provider = "deterministic".to_string();
    template
        .set_parameter("temperature".to_string(), 1.0)
        .unwrap();
    assert!(template.is_deterministic());
}

#[test]
fn test_template_tag_management() {
    let mut template = TemplateDefinition::new("test", "1.0.0", "{{test}}");

    // Add custom tags
    template.metadata.tags.push("custom".to_string());
    template.metadata.tags.push("test".to_string());

    // Get all tags (including automatic ones)
    let all_tags = template.get_all_tags();
    assert!(all_tags.contains(&"custom".to_string()));
    assert!(all_tags.contains(&"test".to_string()));
    assert!(all_tags.contains(&"deterministic".to_string())); // Auto-added
    assert!(all_tags.contains(&"strict".to_string())); // Auto-added for deterministic_only

    // No duplicates
    template.metadata.tags.push("custom".to_string());
    let all_tags = template.get_all_tags();
    let custom_count = all_tags.iter().filter(|t| *t == "custom").count();
    assert_eq!(custom_count, 1);
}

#[tokio::test]
async fn test_template_engine_error_handling() {
    let engine = TemplateEngine::new();

    // Try to generate with non-existent template
    let input = json!({"test": "value"});
    let result = engine.generate("non_existent", input).await;
    assert!(result.is_err());
}

#[test]
fn test_template_inheritance() {
    let mut base = TemplateDefinition::new("base", "1.0.0", "Base: {{content}}");
    let mut child = TemplateDefinition::new("child", "1.0.0", "Child: {{content}}");

    // Set inheritance
    child.extends = Some("base".to_string());

    // Child should have extends field set
    assert_eq!(child.extends, Some("base".to_string()));
}

#[test]
fn test_template_validation_rules() {
    let mut template = TemplateDefinition::new("test", "1.0.0", "{{test}}");

    // Set validation rules
    template.validation.deterministic_only = true;
    template.validation.required_fields = vec!["field1".to_string(), "field2".to_string()];
    template.validation.min_length = Some(10);
    template.validation.max_length = Some(1000);

    // Validate with non-deterministic settings should fail
    template.metadata.provider = "openai".to_string();
    template
        .set_parameter("temperature".to_string(), 0.5)
        .unwrap();
    assert!(template.validate().is_err());

    // Make deterministic again
    template
        .set_parameter("temperature".to_string(), 0.0)
        .unwrap();
    assert!(template.validate().is_ok());
}

#[cfg(feature = "quality-proxy")]
#[test]
fn test_template_quality_enforcement() {
    use pdmt::template::definition::{PmatConfig, QualityEnforcement, QualityMode};
    use std::collections::HashMap;

    let mut template = TemplateDefinition::new("test", "1.0.0", "{{test}}");

    let enforcement = QualityEnforcement {
        pmat_config: PmatConfig {
            mode: "strict".to_string(),
            max_complexity: 10,
            allow_satd: false,
            require_docs: true,
            auto_format: true,
            custom_settings: HashMap::new(),
        },
        auto_refactor: true,
        mode: QualityMode::Strict,
        thresholds: HashMap::new(),
    };

    template.quality_enforcement = Some(enforcement);

    // Template should have quality tag
    let tags = template.get_all_tags();
    assert!(tags.contains(&"quality-enforced".to_string()));
}
