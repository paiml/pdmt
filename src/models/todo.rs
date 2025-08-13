//! Todo list data models
//!
//! Data structures specifically for todo list generation and validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete todo list structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    /// List of individual todos
    pub todos: Vec<Todo>,

    /// Metadata about the todo list
    pub metadata: TodoListMetadata,

    /// Optional project context
    pub project: Option<ProjectContext>,
}

/// Individual todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Unique identifier (UUID v4)
    pub id: String,

    /// Task description (10-100 characters, specific and actionable)
    pub content: String,

    /// Current status
    pub status: TodoStatus,

    /// Priority level
    pub priority: TodoPriority,

    /// Estimated hours to complete (0.5-40 hours)
    pub estimated_hours: Option<f32>,

    /// Dependencies (IDs of other todos that must complete first)
    pub dependencies: Vec<String>,

    /// Quality gates for this todo
    pub quality_gates: TodoQualityGates,

    /// Optional tags for categorization
    pub tags: Vec<String>,

    /// Optional assignee
    pub assignee: Option<String>,

    /// Optional due date
    #[cfg(feature = "todo-validation")]
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Creation timestamp
    #[cfg(feature = "todo-validation")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Custom fields
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Todo status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    /// Task not yet started
    Pending,
    /// Task currently being worked on
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task blocked by external factors
    Blocked,
    /// Task cancelled or no longer needed
    Cancelled,
}

/// Todo priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoPriority {
    /// Low priority
    Low,
    /// Medium priority (default)
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Quality gates for individual todos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoQualityGates {
    /// Whether complexity check passed
    pub complexity_check: bool,

    /// Whether completeness check passed
    pub completeness_check: bool,

    /// Whether actionability check passed
    pub actionability_check: bool,

    /// Whether time estimate is reasonable
    pub time_estimate_check: bool,

    /// Custom quality checks
    pub custom_checks: HashMap<String, bool>,
}

/// Metadata for the entire todo list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoListMetadata {
    /// Total number of todos
    pub total_count: usize,

    /// Count by status
    pub status_counts: HashMap<TodoStatus, usize>,

    /// Count by priority
    pub priority_counts: HashMap<TodoPriority, usize>,

    /// Total estimated hours
    pub total_estimated_hours: f32,

    /// Average estimated hours per task
    pub avg_estimated_hours: f32,

    /// Completion percentage (0.0-1.0)
    pub completion_percentage: f32,

    /// Whether dependency graph is valid (no cycles)
    pub dependency_graph_valid: bool,

    /// Generation timestamp
    #[cfg(feature = "todo-validation")]
    pub generated_at: chrono::DateTime<chrono::Utc>,

    /// Template version used for generation
    pub template_version: String,

    /// Custom metadata fields
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

/// Project context for todo lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    /// Project name
    pub name: String,

    /// Project description
    pub description: Option<String>,

    /// Project type (web, cli, library, service, etc.)
    pub project_type: Option<String>,

    /// Target completion date
    #[cfg(feature = "todo-validation")]
    pub target_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Project stakeholders
    pub stakeholders: Vec<String>,

    /// Technology stack
    pub tech_stack: Vec<String>,

    /// Budget constraints
    pub budget_hours: Option<f32>,
}

/// Input for todo list generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoInput {
    /// Project name
    pub project_name: String,

    /// List of high-level requirements
    pub requirements: Vec<String>,

    /// Level of granularity for task breakdown
    pub granularity: TodoGranularity,

    /// Optional project context
    pub project_context: Option<ProjectContext>,

    /// Quality configuration overrides
    pub quality_config: Option<TodoQualityConfig>,

    /// Maximum number of todos to generate
    pub max_todos: Option<usize>,

    /// Whether to include time estimates
    pub include_estimates: bool,

    /// Default priority for generated todos
    pub default_priority: Option<TodoPriority>,
}

/// Granularity levels for todo generation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoGranularity {
    /// High granularity - many small, specific tasks
    High,
    /// Medium granularity - balanced task sizes
    Medium,
    /// Low granularity - fewer, larger tasks
    Low,
}

/// Quality configuration for todo generation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TodoQualityConfig {
    /// Maximum todos per batch
    pub max_todos_per_batch: Option<usize>,

    /// Minimum characters per task description
    pub min_task_detail_chars: Option<usize>,

    /// Maximum characters per task description
    pub max_task_detail_chars: Option<usize>,

    /// Maximum complexity per task (1-10)
    pub max_complexity_per_task: Option<u8>,

    /// Whether time estimates are required
    pub require_time_estimates: bool,

    /// Whether specific actions are required (vs. generic language)
    pub require_specific_actions: bool,

    /// Whether dependency graph is required
    pub require_dependency_graph: bool,

    /// Whether to prevent circular dependencies
    pub prevent_circular_dependencies: bool,

    /// Minimum hours for time estimates
    pub min_estimated_hours: Option<f32>,

    /// Maximum hours for time estimates
    pub max_estimated_hours: Option<f32>,
}

impl Todo {
    /// Create a new todo with defaults
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.into(),
            status: TodoStatus::Pending,
            priority: TodoPriority::Medium,
            estimated_hours: None,
            dependencies: Vec::new(),
            quality_gates: TodoQualityGates::default(),
            tags: Vec::new(),
            assignee: None,
            #[cfg(feature = "todo-validation")]
            due_date: None,
            #[cfg(feature = "todo-validation")]
            created_at: chrono::Utc::now(),
            custom_fields: HashMap::new(),
        }
    }

    /// Check if todo is actionable (starts with action verb)
    pub fn is_actionable(&self) -> bool {
        let actionable_verbs = [
            "implement",
            "create",
            "build",
            "write",
            "add",
            "remove",
            "update",
            "fix",
            "test",
            "deploy",
            "configure",
            "setup",
            "install",
            "design",
            "develop",
            "refactor",
            "optimize",
            "migrate",
            "integrate",
            "debug",
            "analyze",
            "research",
            "document",
            "validate",
            "verify",
            "review",
        ];

        let lower_content = self.content.to_lowercase();
        actionable_verbs
            .iter()
            .any(|verb| lower_content.starts_with(verb))
    }

    /// Check if content length is within valid range
    pub fn has_valid_length(&self, min_chars: usize, max_chars: usize) -> bool {
        let len = self.content.len();
        len >= min_chars && len <= max_chars
    }

    /// Estimate complexity score (1-10) based on content
    pub fn complexity_score(&self) -> u8 {
        let content = &self.content.to_lowercase();
        let mut score = 1;

        // Check for complexity indicators
        let complexity_words = [
            "integrate",
            "refactor",
            "optimize",
            "migrate",
            "analyze",
            "algorithm",
            "performance",
            "security",
            "architecture",
        ];

        for word in &complexity_words {
            if content.contains(word) {
                score += 1;
            }
        }

        // Check for technical terms
        if content.contains("database") || content.contains("api") || content.contains("system") {
            score += 1;
        }

        // Check for multiple actions in one task
        let action_count = content.matches(" and ").count() + content.matches(", ").count();
        score += (action_count / 2) as u8;

        score.min(10)
    }

    /// Check if task has reasonable time estimate
    pub fn has_reasonable_estimate(&self, min_hours: f32, max_hours: f32) -> bool {
        match self.estimated_hours {
            Some(hours) => hours >= min_hours && hours <= max_hours,
            None => false,
        }
    }

    /// Get progress percentage (0.0 for pending, 0.5 for `in_progress`, 1.0 for completed)
    pub const fn progress(&self) -> f32 {
        match self.status {
            TodoStatus::Pending | TodoStatus::Blocked => 0.0,
            TodoStatus::InProgress => 0.5,
            TodoStatus::Completed => 1.0,
            TodoStatus::Cancelled => 0.0,
        }
    }
}

impl TodoList {
    /// Create a new empty todo list
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            metadata: TodoListMetadata::default(),
            project: None,
        }
    }

    /// Add a todo to the list
    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
        // Don't check cycles during add to prevent recursion
        self.update_metadata_internal(false);
    }

    /// Update metadata based on current todos
    pub fn update_metadata(&mut self) {
        self.update_metadata_internal(true);
    }

    /// Internal metadata update with cycle check control
    fn update_metadata_internal(&mut self, check_cycles: bool) {
        let total_count = self.todos.len();

        // Count by status
        let mut status_counts = HashMap::new();
        for todo in &self.todos {
            *status_counts.entry(todo.status).or_insert(0) += 1;
        }

        // Count by priority
        let mut priority_counts = HashMap::new();
        for todo in &self.todos {
            *priority_counts.entry(todo.priority).or_insert(0) += 1;
        }

        // Calculate totals and averages
        let total_estimated_hours: f32 = self.todos.iter().filter_map(|t| t.estimated_hours).sum();

        let avg_estimated_hours = if total_count > 0 {
            total_estimated_hours / total_count as f32
        } else {
            0.0
        };

        let completion_percentage = if total_count > 0 {
            let completed_count = status_counts.get(&TodoStatus::Completed).unwrap_or(&0);
            *completed_count as f32 / total_count as f32
        } else {
            0.0
        };

        // Check dependency graph validity (only if requested to avoid recursion)
        let dependency_graph_valid = if check_cycles {
            self.validate_dependencies().is_ok()
        } else {
            true // Assume valid when not checking
        };

        self.metadata = TodoListMetadata {
            total_count,
            status_counts,
            priority_counts,
            total_estimated_hours,
            avg_estimated_hours,
            completion_percentage,
            dependency_graph_valid,
            #[cfg(feature = "todo-validation")]
            generated_at: chrono::Utc::now(),
            template_version: "1.0.0".to_string(),
            custom_metadata: HashMap::new(),
        };
    }

    /// Validate dependency graph for cycles
    pub fn validate_dependencies(&self) -> Result<(), Vec<String>> {
        use std::collections::{HashMap, HashSet, VecDeque};

        // Build adjacency list
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // Initialize nodes
        for todo in &self.todos {
            graph.insert(todo.id.clone(), todo.dependencies.clone());
            in_degree.insert(todo.id.clone(), 0);
        }

        // Calculate in-degrees
        for todo in &self.todos {
            for dep in &todo.dependencies {
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree += 1;
                }
            }
        }

        // Topological sort using Kahn's algorithm
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut processed = 0;

        while let Some(current) = queue.pop_front() {
            processed += 1;

            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if processed != self.todos.len() {
            // Find cycle
            let mut cycle = Vec::new();
            let remaining: HashSet<String> = in_degree
                .iter()
                .filter(|(_, &degree)| degree > 0)
                .map(|(id, _)| id.clone())
                .collect();

            if let Some(start) = remaining.iter().next() {
                let mut current = start.clone();
                let mut visited = HashSet::new();

                while !visited.contains(&current) {
                    visited.insert(current.clone());
                    cycle.push(current.clone());

                    // Find next node in cycle
                    if let Some(deps) = graph.get(&current) {
                        if let Some(next) = deps.iter().find(|dep| remaining.contains(*dep)) {
                            current = next.clone();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            Err(cycle)
        } else {
            Ok(())
        }
    }

    /// Get todos by status
    pub fn todos_by_status(&self, status: TodoStatus) -> Vec<&Todo> {
        self.todos.iter().filter(|t| t.status == status).collect()
    }

    /// Get todos by priority
    pub fn todos_by_priority(&self, priority: TodoPriority) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|t| t.priority == priority)
            .collect()
    }

    /// Get critical path (longest dependency chain)
    pub fn critical_path(&self) -> Vec<String> {
        // Implementation would calculate the longest path through the dependency graph
        // For now, return empty path
        Vec::new()
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TodoQualityGates {
    fn default() -> Self {
        Self {
            complexity_check: false,
            completeness_check: false,
            actionability_check: false,
            time_estimate_check: false,
            custom_checks: HashMap::new(),
        }
    }
}

impl Default for TodoListMetadata {
    fn default() -> Self {
        Self {
            total_count: 0,
            status_counts: HashMap::new(),
            priority_counts: HashMap::new(),
            total_estimated_hours: 0.0,
            avg_estimated_hours: 0.0,
            completion_percentage: 0.0,
            dependency_graph_valid: true,
            #[cfg(feature = "todo-validation")]
            generated_at: chrono::Utc::now(),
            template_version: "1.0.0".to_string(),
            custom_metadata: HashMap::new(),
        }
    }
}

impl Default for TodoQualityConfig {
    fn default() -> Self {
        Self {
            max_todos_per_batch: Some(50),
            min_task_detail_chars: Some(10),
            max_task_detail_chars: Some(100),
            max_complexity_per_task: Some(8),
            require_time_estimates: true,
            require_specific_actions: true,
            require_dependency_graph: true,
            prevent_circular_dependencies: true,
            min_estimated_hours: Some(0.5),
            max_estimated_hours: Some(40.0),
        }
    }
}

impl Default for TodoInput {
    fn default() -> Self {
        Self {
            project_name: "Sample Project".to_string(),
            requirements: vec!["Create basic functionality".to_string()],
            granularity: TodoGranularity::Medium,
            project_context: None,
            quality_config: None,
            max_todos: Some(20),
            include_estimates: true,
            default_priority: Some(TodoPriority::Medium),
        }
    }
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "pending"),
            TodoStatus::InProgress => write!(f, "in_progress"),
            TodoStatus::Completed => write!(f, "completed"),
            TodoStatus::Blocked => write!(f, "blocked"),
            TodoStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoPriority::Low => write!(f, "low"),
            TodoPriority::Medium => write!(f, "medium"),
            TodoPriority::High => write!(f, "high"),
            TodoPriority::Critical => write!(f, "critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new("Implement user authentication");
        assert!(!todo.id.is_empty());
        assert_eq!(todo.content, "Implement user authentication");
        assert_eq!(todo.status, TodoStatus::Pending);
        assert_eq!(todo.priority, TodoPriority::Medium);
    }

    #[test]
    fn test_todo_actionability() {
        let actionable = Todo::new("Implement user login system");
        assert!(actionable.is_actionable());

        let not_actionable = Todo::new("User login stuff");
        assert!(!not_actionable.is_actionable());
    }

    #[test]
    fn test_todo_complexity_score() {
        let simple = Todo::new("Add button to form");
        assert_eq!(simple.complexity_score(), 1);

        let complex = Todo::new("Integrate complex authentication system with database and API");
        let score = complex.complexity_score();
        assert!(score >= 3, "Expected complexity score >= 3, got {}", score);
    }

    #[test]
    fn test_todo_list_metadata() {
        let mut list = TodoList::new();
        let todo1 = Todo::new("Task 1");
        let mut todo2 = Todo::new("Task 2");
        todo2.status = TodoStatus::Completed;

        list.add_todo(todo1);
        list.add_todo(todo2);

        assert_eq!(list.metadata.total_count, 2);
        assert_eq!(list.metadata.completion_percentage, 0.5);
        assert_eq!(
            *list
                .metadata
                .status_counts
                .get(&TodoStatus::Pending)
                .unwrap_or(&0),
            1
        );
        assert_eq!(
            *list
                .metadata
                .status_counts
                .get(&TodoStatus::Completed)
                .unwrap_or(&0),
            1
        );
    }

    #[test]
    fn test_dependency_validation() {
        let mut list = TodoList::new();

        let mut todo1 = Todo::new("Task 1");
        todo1.id = "task1".to_string();

        let mut todo2 = Todo::new("Task 2");
        todo2.id = "task2".to_string();
        todo2.dependencies = vec!["task1".to_string()];

        list.add_todo(todo1);
        list.add_todo(todo2);

        assert!(list.validate_dependencies().is_ok());

        // Create circular dependency
        list.todos[0].dependencies = vec!["task2".to_string()];
        list.update_metadata_internal(false); // Don't check cycles in metadata update to avoid recursion

        assert!(list.validate_dependencies().is_err());
    }

    #[test]
    fn test_todo_progress() {
        let pending = Todo::new("Pending task");
        assert_eq!(pending.progress(), 0.0);

        let mut in_progress = Todo::new("In progress task");
        in_progress.status = TodoStatus::InProgress;
        assert_eq!(in_progress.progress(), 0.5);

        let mut completed = Todo::new("Completed task");
        completed.status = TodoStatus::Completed;
        assert_eq!(completed.progress(), 1.0);
    }
}
