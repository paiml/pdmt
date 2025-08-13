//! Integration tests for PDMT

use pdmt::{models::todo::TodoInput, TemplateEngine};
use serde_json::json;

#[tokio::test]
async fn test_basic_todo_generation() {
    let mut engine = TemplateEngine::new();
    engine.load_builtin_templates().await.unwrap();

    let input = TodoInput {
        project_name: "Test Project".to_string(),
        requirements: vec!["Create user auth".to_string(), "Build API".to_string()],
        granularity: pdmt::models::todo::TodoGranularity::High,
        project_context: None,
        quality_config: None,
        max_todos: Some(10),
        include_estimates: true,
        default_priority: None,
    };

    let result = engine.generate("todo_list", input).await.unwrap();
    assert!(!result.content.is_empty());
    assert_eq!(result.template_id, "todo_list");
}

#[tokio::test]
async fn test_deterministic_generation() {
    let mut engine = TemplateEngine::new();
    engine.load_builtin_templates().await.unwrap();

    let input = json!({
        "project_name": "Test",
        "requirements": ["task1", "task2"]
    });

    let result1 = engine.generate("todo_list", &input).await.unwrap();
    let result2 = engine.generate("todo_list", &input).await.unwrap();

    assert_eq!(result1.content, result2.content);
}
