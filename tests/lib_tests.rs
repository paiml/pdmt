use pdmt::models::todo::{TodoGranularity, TodoStatus};

#[cfg(feature = "todo-validation")]
#[test]
fn test_template_id_validation() {
    use pdmt::utils::validate_template_id;
    
    // Valid template IDs
    assert!(validate_template_id("valid_template").is_ok());
    assert!(validate_template_id("template123").is_ok());
    assert!(validate_template_id("my_template").is_ok());
    assert!(validate_template_id("template_with_underscores").is_ok());
    
    // Invalid template IDs
    assert!(validate_template_id("").is_err());
    assert!(validate_template_id("template with spaces").is_err());
    assert!(validate_template_id("template@invalid").is_err());
    assert!(validate_template_id("template-with-dashes").is_err());
    assert!(validate_template_id("template.with.dots").is_err());
    
    // Edge cases
    let long_id = "a".repeat(300);
    assert!(validate_template_id(&long_id).is_err());
}

#[cfg(feature = "todo-validation")]
#[test]
fn test_utils_functions() {
    use pdmt::utils::{current_timestamp, generate_content_id};
    
    // Test timestamp generation
    let ts1 = current_timestamp();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let ts2 = current_timestamp();
    assert!(ts2 > ts1);
    
    // Test content ID generation
    let id1 = generate_content_id();
    let id2 = generate_content_id();
    assert_ne!(id1, id2);
    assert!(id1.len() > 0);
    assert!(id2.len() > 0);
    // UUIDs should be 36 characters with dashes
    assert_eq!(id1.len(), 36);
    assert_eq!(id2.len(), 36);
}

#[test]
fn test_todo_granularity_values() {
    // Test that granularity enum values can be constructed
    let _low = TodoGranularity::Low;
    let _medium = TodoGranularity::Medium;
    let _high = TodoGranularity::High;
}

#[test]
fn test_todo_status_display() {
    assert_eq!(TodoStatus::Pending.to_string(), "pending");
    assert_eq!(TodoStatus::InProgress.to_string(), "in_progress");
    assert_eq!(TodoStatus::Completed.to_string(), "completed");
    assert_eq!(TodoStatus::Blocked.to_string(), "blocked");
    assert_eq!(TodoStatus::Cancelled.to_string(), "cancelled");
}