use pdmt::models::content::{GeneratedContent, ContentFormat};
use pdmt::models::quality::QualityReport;
use serde_json::json;
use std::str::FromStr;

#[test]
fn test_content_format_yaml_to_json() {
    let yaml_content = r#"
todos:
  - id: task1
    content: Do something
    priority: high
"#;
    
    let content = GeneratedContent::new(
        "test".to_string(),
        yaml_content.to_string(),
        json!({}),
    );
    
    let json_content = content.as_format(ContentFormat::Json).unwrap();
    assert!(json_content.contains("\"todos\""));
    assert!(json_content.contains("\"task1\""));
}

#[test]
fn test_content_format_yaml_to_markdown() {
    let yaml_content = r#"
title: Test Document
items:
  - item1
  - item2
metadata:
  author: test
  version: 1.0
"#;
    
    let content = GeneratedContent::new(
        "test".to_string(),
        yaml_content.to_string(),
        json!({}),
    );
    
    let markdown_content = content.as_format(ContentFormat::Markdown).unwrap();
    assert!(markdown_content.contains("title"));
    assert!(markdown_content.contains("item1"));
}

#[test]
fn test_content_format_yaml_to_text() {
    let yaml_content = r#"
title: Test Document
content: This is the content
"#;
    
    let content = GeneratedContent::new(
        "test".to_string(),
        yaml_content.to_string(),
        json!({}),
    );
    
    let text_content = content.as_format(ContentFormat::Text).unwrap();
    assert!(text_content.contains("Test Document"));
    assert!(text_content.contains("This is the content"));
}

#[test]
fn test_content_quality_checks() {
    let mut content = GeneratedContent::new(
        "test".to_string(),
        "test content".to_string(),
        json!({}),
    );
    
    // Test quality report
    assert!(!content.has_quality_issues());
    
    content.quality_report = Some(QualityReport {
        passed: false,
        violations: vec![],
        suggestions: vec!["Improve quality".to_string()],
    });
    
    assert!(content.has_quality_issues());
}

#[test]
fn test_content_processing_duration() {
    let mut content = GeneratedContent::new(
        "test".to_string(),
        "test".to_string(),
        json!({}),
    );
    
    content.metadata.processing_time_ms = 1500;
    let duration = content.processing_duration();
    assert_eq!(duration.as_millis(), 1500);
}

#[test]
fn test_content_format_parsing() {
    assert_eq!(ContentFormat::from_str("yaml").unwrap(), ContentFormat::Yaml);
    assert_eq!(ContentFormat::from_str("json").unwrap(), ContentFormat::Json);
    assert_eq!(ContentFormat::from_str("text").unwrap(), ContentFormat::Text);
    assert_eq!(ContentFormat::from_str("markdown").unwrap(), ContentFormat::Markdown);
    assert!(ContentFormat::from_str("unknown").is_err());
    
    assert_eq!(ContentFormat::Yaml.to_string(), "yaml");
    assert_eq!(ContentFormat::Json.to_string(), "json");
}

#[test]
fn test_content_with_complex_yaml() {
    let yaml_content = r#"
project:
  name: Test Project
  tasks:
    - id: task1
      dependencies: [task2, task3]
    - id: task2
      dependencies: []
metadata:
  version: 1.0.0
"#;
    
    let content = GeneratedContent::new(
        "test".to_string(),
        yaml_content.to_string(),
        json!({}),
    );
    
    let json_result = content.as_format(ContentFormat::Json).unwrap();
    assert!(json_result.contains("\"project\""));
    assert!(json_result.contains("\"dependencies\""));
}

#[test]
fn test_content_metadata() {
    let content = GeneratedContent::new(
        "test_template".to_string(),
        "content".to_string(),
        json!({"input": "data"}),
    );
    
    assert_eq!(content.template_id, "test_template");
    assert_eq!(content.content, "content");
    assert_eq!(content.input_data, json!({"input": "data"}));
    
    // Check metadata defaults
    assert!(content.metadata.is_deterministic);
    assert_eq!(content.metadata.processing_time_ms, 0);
}