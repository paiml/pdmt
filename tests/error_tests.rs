use pdmt::error::{Error, TemplateError, ValidationError};

#[test]
fn test_error_creation_and_display() {
    let err = Error::InvalidInput("test input".to_string());
    assert!(err.to_string().contains("test input"));

    let err = Error::Config("config error".to_string());
    assert!(err.to_string().contains("config error"));

    let err = Error::Internal("internal error".to_string());
    assert!(err.to_string().contains("internal error"));
}

#[test]
fn test_template_error_creation() {
    let err = TemplateError::not_found("test_template");
    assert!(err.to_string().contains("test_template"));

    let err = TemplateError::invalid_definition("bad template");
    assert!(err.to_string().contains("bad template"));

    let err = TemplateError::size_limit(1000, 500);
    assert!(err.to_string().contains("1000"));
    assert!(err.to_string().contains("500"));
}

#[test]
fn test_error_conversions() {
    let json_err = serde_json::from_str::<i32>("not_a_number").unwrap_err();
    let err: Error = json_err.into();
    assert!(matches!(err, Error::Serialization(_)));

    let yaml_str = "invalid: [yaml";
    let yaml_err = serde_yaml::from_str::<serde_yaml::Value>(yaml_str).unwrap_err();
    let err: Error = yaml_err.into();
    assert!(matches!(err, Error::Serialization(_)));
}

#[test]
fn test_error_helpers() {
    let err = Error::invalid_input("bad input");
    assert!(matches!(err, Error::InvalidInput(_)));

    let err = Error::config("bad config");
    assert!(matches!(err, Error::Config(_)));

    let err = Error::internal("internal issue");
    assert!(matches!(err, Error::Internal(_)));
}

#[test]
fn test_validation_error_creation() {
    let err = ValidationError::MissingField {
        field: "test_field".to_string(),
    };
    assert!(err.to_string().contains("test_field"));

    let err = ValidationError::InvalidValue {
        field: "test".to_string(),
        reason: "too short".to_string(),
    };
    assert!(err.to_string().contains("test"));
    assert!(err.to_string().contains("too short"));

    let err = ValidationError::StructureError {
        reason: "bad structure".to_string(),
    };
    assert!(err.to_string().contains("bad structure"));
}

#[cfg(feature = "quality-proxy")]
#[test]
fn test_quality_error_creation() {
    use pdmt::error::{QualityError, QualityViolation, Severity};

    let violation = QualityViolation {
        violation_type: "complexity".to_string(),
        severity: Severity::Error,
        location: Some("file.rs:10".to_string()),
        message: "Too complex".to_string(),
        suggestion: Some("Simplify".to_string()),
    };

    let violation_with_location = violation.clone().with_location("new_file.rs:20");
    assert_eq!(
        violation_with_location.location,
        Some("new_file.rs:20".to_string())
    );

    let violation_with_suggestion = violation.clone().with_suggestion("Refactor the code");
    assert_eq!(
        violation_with_suggestion.suggestion,
        Some("Refactor the code".to_string())
    );

    let err = QualityError::QualityGateFailed {
        violations: vec![violation],
        suggestions: vec!["Fix issues".to_string()],
    };
    assert!(err.to_string().contains("Quality gate failed"));
}

#[cfg(feature = "mcp-tools")]
#[test]
fn test_mcp_error_creation() {
    use pdmt::error::McpError;

    let err = McpError::InvalidRequest {
        message: "bad request".to_string(),
    };
    assert!(err.to_string().contains("bad request"));

    let err = McpError::ToolNotFound {
        name: "missing_tool".to_string(),
    };
    assert!(err.to_string().contains("missing_tool"));
}

#[cfg(feature = "todo-validation")]
#[test]
fn test_todo_validation_error() {
    use pdmt::error::TodoValidationError;

    let err = TodoValidationError::NotActionable {
        content: "vague task".to_string(),
    };
    assert!(err.to_string().contains("vague task"));

    let err = TodoValidationError::TooVague {
        content: "do it".to_string(),
        min_chars: 10,
    };
    assert!(err.to_string().contains("do it"));

    let err = TodoValidationError::CircularDependency {
        cycle: vec![
            "task1".to_string(),
            "task2".to_string(),
            "task1".to_string(),
        ],
    };
    assert!(err.to_string().contains("Circular dependency"));
}
