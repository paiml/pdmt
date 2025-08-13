use pdmt::models::todo::{Todo, TodoList, TodoStatus, TodoPriority};

#[test]
fn test_todo_list_edge_cases() {
    let mut todo_list = TodoList::new();
    
    // Test empty list
    assert_eq!(todo_list.todos.len(), 0);
    assert_eq!(todo_list.metadata.total_count, 0);
    
    // Add some todos
    let mut todo1 = Todo::new("Task 1");
    todo1.id = "task1".to_string();
    let mut todo2 = Todo::new("Task 2");
    todo2.id = "task2".to_string();
    todo2.dependencies = vec!["task1".to_string()];
    
    todo_list.add_todo(todo1.clone());
    todo_list.add_todo(todo2.clone());
    
    // Test list operations
    assert_eq!(todo_list.todos.len(), 2);
    assert_eq!(todo_list.metadata.total_count, 2);
    
    // Test finding todo by ID
    let found = todo_list.todos.iter().find(|t| t.id == "task1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().content, "Task 1");
    
    // Test modifying todo
    for todo in &mut todo_list.todos {
        if todo.id == "task1" {
            todo.status = TodoStatus::Completed;
        }
    }
    
    let completed = todo_list.todos.iter().find(|t| t.id == "task1");
    assert_eq!(completed.unwrap().status, TodoStatus::Completed);
}

#[test]
fn test_todo_list_cycle_detection() {
    let mut todo_list = TodoList::new();
    
    // Create a circular dependency
    let mut todo1 = Todo::new("Task 1");
    todo1.id = "task1".to_string();
    todo1.dependencies = vec!["task3".to_string()];
    
    let mut todo2 = Todo::new("Task 2");
    todo2.id = "task2".to_string();
    todo2.dependencies = vec!["task1".to_string()];
    
    let mut todo3 = Todo::new("Task 3");
    todo3.id = "task3".to_string();
    todo3.dependencies = vec!["task2".to_string()];
    
    todo_list.add_todo(todo1);
    todo_list.add_todo(todo2);
    todo_list.add_todo(todo3);
    
    // Update metadata to check for cycles
    todo_list.update_metadata();
    
    // Should detect the cycle (graph should be invalid)
    assert!(!todo_list.metadata.dependency_graph_valid);
}

#[test]
fn test_todo_state_transitions() {
    let mut todo = Todo::new("Test task");
    
    // Test setting due date
    use chrono::{Utc};
    let due_date = Utc::now();
    todo.due_date = Some(due_date);
    assert!(todo.due_date.is_some());
    
    // Test marking as blocked
    todo.status = TodoStatus::Blocked;
    assert_eq!(todo.status, TodoStatus::Blocked);
    
    // Test marking as cancelled
    todo.status = TodoStatus::Cancelled;
    assert_eq!(todo.status, TodoStatus::Cancelled);
    
    // Test marking as complete
    todo.status = TodoStatus::Completed;
    assert_eq!(todo.status, TodoStatus::Completed);
}

#[test]
fn test_todo_priority_operations() {
    let mut todo = Todo::new("Test task");
    
    // Test priority string conversion
    todo.priority = TodoPriority::Critical;
    assert_eq!(todo.priority.to_string(), "critical");
    
    todo.priority = TodoPriority::High;
    assert_eq!(todo.priority.to_string(), "high");
    
    todo.priority = TodoPriority::Medium;
    assert_eq!(todo.priority.to_string(), "medium");
    
    todo.priority = TodoPriority::Low;
    assert_eq!(todo.priority.to_string(), "low");
}

#[test]
fn test_todo_list_batch_operations() {
    let mut todo_list = TodoList::new();
    
    // Add multiple todos
    for i in 0..5 {
        let mut todo = Todo::new(format!("Task {}", i));
        todo.id = format!("task{}", i);
        if i > 0 {
            todo.dependencies = vec![format!("task{}", i - 1)];
        }
        todo_list.add_todo(todo);
    }
    
    // Check metadata
    assert_eq!(todo_list.metadata.total_count, 5);
    assert!(todo_list.metadata.dependency_graph_valid);
    
    // Get todos by status
    let pending: Vec<_> = todo_list.todos.iter()
        .filter(|t| t.status == TodoStatus::Pending)
        .collect();
    assert_eq!(pending.len(), 5);
    
    // Mark some as completed
    for todo in &mut todo_list.todos {
        if todo.id == "task0" || todo.id == "task1" {
            todo.status = TodoStatus::Completed;
        }
    }
    
    let completed: Vec<_> = todo_list.todos.iter()
        .filter(|t| t.status == TodoStatus::Completed)
        .collect();
    assert_eq!(completed.len(), 2);
}