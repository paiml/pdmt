use pdmt::models::todo::{Todo, TodoList, TodoPriority, TodoQualityConfig, TodoStatus};
use pdmt::validators::todo::{IssueCategory, IssueSeverity, TodoValidator};

#[test]
fn test_validator_with_custom_config() {
    let config = TodoQualityConfig {
        max_todos_per_batch: Some(5),
        min_task_detail_chars: Some(20),
        max_task_detail_chars: Some(200),
        max_complexity_per_task: Some(5),
        require_time_estimates: true,
        require_specific_actions: true,
        require_dependency_graph: false,
        prevent_circular_dependencies: false,
        min_estimated_hours: Some(1.0),
        max_estimated_hours: Some(20.0),
    };

    let validator = TodoValidator::with_config(config);
    let mut todo_list = TodoList::new();

    // Add a todo that violates multiple rules
    let mut bad_todo = Todo::new("stuff");
    bad_todo.estimated_hours = Some(0.1); // Too low
    todo_list.add_todo(bad_todo);

    let result = validator.validate_todo_list(&todo_list);
    assert!(!result.is_valid);

    // Check for specific issues
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Actionability));
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Completeness));
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::TimeEstimate));
}

#[test]
fn test_validator_structure_checks() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    // Add todos with duplicate IDs
    let mut todo1 = Todo::new("Task 1");
    todo1.id = "duplicate_id".to_string();
    let mut todo2 = Todo::new("Task 2");
    todo2.id = "duplicate_id".to_string();

    todo_list.add_todo(todo1);
    todo_list.add_todo(todo2);

    let result = validator.validate_todo_list(&todo_list);
    assert!(
        result
            .issues
            .iter()
            .any(|i| i.category == IssueCategory::Structure
                && i.message.contains("Duplicate todo ID"))
    );
}

#[test]
fn test_validator_dependency_checks() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    let mut todo1 = Todo::new("Implement feature");
    todo1.id = "task1".to_string();
    todo1.dependencies = vec!["non_existent".to_string()];

    let mut todo2 = Todo::new("Test feature");
    todo2.id = "task2".to_string();
    todo2.dependencies = vec!["task2".to_string()]; // Self-dependency

    todo_list.add_todo(todo1);
    todo_list.add_todo(todo2);

    let result = validator.validate_todo_list(&todo_list);

    // Check for dependency not found
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Dependencies && i.message.contains("not found")));

    // Check for self-dependency
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Dependencies
            && i.message.contains("depends on itself")));
}

#[test]
fn test_validator_generic_language_detection() {
    let config = TodoQualityConfig {
        require_specific_actions: true,
        ..TodoQualityConfig::default()
    };

    let validator = TodoValidator::with_config(config);
    let mut todo_list = TodoList::new();

    // Add todos with generic language
    todo_list.add_todo(Todo::new("Fix stuff in the thing"));
    todo_list.add_todo(Todo::new("Handle something"));
    todo_list.add_todo(Todo::new("Do item processing"));

    let result = validator.validate_todo_list(&todo_list);

    // Should have warnings about generic language
    let generic_issues: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("generic language"))
        .collect();

    assert!(!generic_issues.is_empty());
}

#[test]
fn test_validator_quality_score_calculation() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    // Add high-quality todos
    let mut good_todo1 = Todo::new("Implement user authentication with OAuth2");
    good_todo1.estimated_hours = Some(8.0);
    good_todo1.priority = TodoPriority::High;

    let mut good_todo2 = Todo::new("Create REST API endpoints for user management");
    good_todo2.estimated_hours = Some(6.0);
    good_todo2.dependencies = vec![good_todo1.id.clone()];

    todo_list.add_todo(good_todo1);
    todo_list.add_todo(good_todo2);

    let result = validator.validate_todo_list(&todo_list);

    // Should have high quality metrics
    assert_eq!(result.metrics.actionable_count, 2);
    assert_eq!(result.metrics.estimated_count, 2);
    assert!(result.metrics.total_estimated_hours > 0.0);
    assert!(!result.metrics.dependency_metrics.has_cycles);
}

#[test]
fn test_validator_empty_list() {
    let validator = TodoValidator::new();
    let todo_list = TodoList::new();

    let result = validator.validate_todo_list(&todo_list);

    // Empty list should have structure error
    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Structure && i.message.contains("empty")));
}

#[test]
fn test_validator_max_todos_limit() {
    let config = TodoQualityConfig {
        max_todos_per_batch: Some(3),
        ..TodoQualityConfig::default()
    };

    let validator = TodoValidator::with_config(config);
    let mut todo_list = TodoList::new();

    // Add more todos than limit
    for i in 0..5 {
        todo_list.add_todo(Todo::new(format!("Task {}", i)));
    }

    let result = validator.validate_todo_list(&todo_list);

    assert!(result
        .issues
        .iter()
        .any(|i| i.severity == IssueSeverity::Error && i.message.contains("exceeds maximum")));
}

#[test]
fn test_validator_suggestions() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    // Add todos that need improvement
    let mut todo = Todo::new("thing to do");
    todo.estimated_hours = None;
    todo_list.add_todo(todo);

    let result = validator.validate_todo_list(&todo_list);

    // Should have suggestions
    assert!(!result.suggestions.is_empty());
    assert!(result.suggestions.iter().any(|s| s.contains("actionable")));
}

#[test]
fn test_validator_dependency_depth() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    // Create a chain of dependencies
    let mut todo1 = Todo::new("Task 1");
    todo1.id = "t1".to_string();

    let mut todo2 = Todo::new("Task 2");
    todo2.id = "t2".to_string();
    todo2.dependencies = vec!["t1".to_string()];

    let mut todo3 = Todo::new("Task 3");
    todo3.id = "t3".to_string();
    todo3.dependencies = vec!["t2".to_string()];

    let mut todo4 = Todo::new("Task 4");
    todo4.id = "t4".to_string();
    todo4.dependencies = vec!["t3".to_string()];

    todo_list.add_todo(todo1);
    todo_list.add_todo(todo2);
    todo_list.add_todo(todo3);
    todo_list.add_todo(todo4);

    let result = validator.validate_todo_list(&todo_list);

    // Should calculate depth correctly
    assert_eq!(result.metrics.dependency_metrics.max_depth, 4);
    assert_eq!(result.metrics.dependency_metrics.critical_path_length, 4);
    assert!(!result.metrics.dependency_metrics.has_cycles);
}

#[test]
fn test_validator_duplicate_content() {
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();

    // Add todos with same content (different case/spacing)
    todo_list.add_todo(Todo::new("Implement feature X"));
    todo_list.add_todo(Todo::new("  implement FEATURE x  "));

    let result = validator.validate_todo_list(&todo_list);

    assert!(result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Structure
            && i.message.contains("Duplicate todo content")));
}
