#![no_main]
use libfuzzer_sys::fuzz_target;
use pdmt::models::todo::{Todo, TodoList, TodoStatus, TodoPriority};
use pdmt::validators::todo::TodoValidator;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() || data.len() > 10000 {
        return;
    }
    
    let validator = TodoValidator::new();
    let mut todo_list = TodoList::new();
    
    // Create todos from fuzz data
    let mut i = 0;
    while i < data.len() && todo_list.todos.len() < 100 {
        // Create todo content from data
        let end = (i + 50).min(data.len());
        let content = String::from_utf8_lossy(&data[i..end])
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .take(100)
            .collect::<String>();
        
        if content.len() >= 5 {
            let mut todo = Todo::new(content);
            
            // Set random properties from data
            if i < data.len() {
                todo.status = match data[i] % 5 {
                    0 => TodoStatus::Pending,
                    1 => TodoStatus::InProgress,
                    2 => TodoStatus::Completed,
                    3 => TodoStatus::Blocked,
                    _ => TodoStatus::Cancelled,
                };
            }
            
            if i + 1 < data.len() {
                todo.priority = match data[i + 1] % 4 {
                    0 => TodoPriority::Low,
                    1 => TodoPriority::Medium,
                    2 => TodoPriority::High,
                    _ => TodoPriority::Critical,
                };
            }
            
            if i + 2 < data.len() {
                todo.estimated_hours = Some((data[i + 2] as f32) / 10.0);
            }
            
            // Add some dependencies
            if i + 3 < data.len() && data[i + 3] % 3 == 0 && !todo_list.todos.is_empty() {
                let dep_idx = data[i + 3] as usize % todo_list.todos.len();
                todo.dependencies.push(todo_list.todos[dep_idx].id.clone());
            }
            
            todo_list.add_todo(todo);
        }
        
        i += 10;
    }
    
    // Validate the todo list
    let _result = validator.validate_todo_list(&todo_list);
});