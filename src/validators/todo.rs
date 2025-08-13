//! Todo list validation
//!
//! Specialized validators for todo list content with quality enforcement.

// Validation error types used in validator implementation
use crate::models::todo::{Todo, TodoList, TodoQualityConfig};
use std::collections::{HashMap, HashSet};

/// Validator for todo list content
#[derive(Debug, Clone)]
pub struct TodoValidator {
    config: TodoQualityConfig,
}

/// Validation result with details
#[derive(Debug, Clone)]
pub struct TodoValidationResult {
    /// Whether validation passed
    pub is_valid: bool,

    /// List of validation issues
    pub issues: Vec<ValidationIssue>,

    /// Quality metrics
    pub metrics: TodoMetrics,

    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Individual validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Issue severity
    pub severity: IssueSeverity,

    /// Issue category
    pub category: IssueCategory,

    /// Todo ID (if applicable)
    pub todo_id: Option<String>,

    /// Human-readable message
    pub message: String,

    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Must be fixed
    Error,
    /// Should be fixed
    Warning,
    /// Nice to fix
    Info,
}

/// Categories of validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    /// Actionability issues
    Actionability,
    /// Completeness issues
    Completeness,
    /// Complexity issues
    Complexity,
    /// Time estimate issues
    TimeEstimate,
    /// Dependency issues
    Dependencies,
    /// Structure issues
    Structure,
    /// Quality gate issues
    QualityGate,
}

/// Todo list quality metrics
#[derive(Debug, Clone)]
pub struct TodoMetrics {
    /// Total number of todos
    pub total_count: usize,

    /// Number of actionable todos
    pub actionable_count: usize,

    /// Number of todos with proper length
    pub proper_length_count: usize,

    /// Number of todos with time estimates
    pub estimated_count: usize,

    /// Number of todos with reasonable complexity
    pub reasonable_complexity_count: usize,

    /// Average complexity score
    pub avg_complexity: f32,

    /// Average task length
    pub avg_task_length: f32,

    /// Total estimated hours
    pub total_estimated_hours: f32,

    /// Dependency graph metrics
    pub dependency_metrics: DependencyMetrics,
}

/// Dependency graph metrics
#[derive(Debug, Clone, Copy)]
pub struct DependencyMetrics {
    /// Number of todos with dependencies
    pub todos_with_dependencies: usize,

    /// Total number of dependency relationships
    pub total_dependencies: usize,

    /// Maximum dependency depth
    pub max_depth: usize,

    /// Whether graph has cycles
    pub has_cycles: bool,

    /// Critical path length
    pub critical_path_length: usize,
}

impl TodoValidator {
    /// Create a new todo validator with default configuration
    pub fn new() -> Self {
        Self {
            config: TodoQualityConfig::default(),
        }
    }

    /// Create a validator with custom configuration
    pub fn with_config(config: TodoQualityConfig) -> Self {
        Self { config }
    }

    /// Validate a complete todo list
    pub fn validate_todo_list(&self, todo_list: &TodoList) -> TodoValidationResult {
        let mut issues = Vec::new();

        // Validate overall structure
        self.validate_structure(todo_list, &mut issues);

        // Validate individual todos
        for todo in &todo_list.todos {
            self.validate_todo(todo, &mut issues);
        }

        // Validate dependencies
        self.validate_dependencies(todo_list, &mut issues);

        // Calculate metrics
        let metrics = self.calculate_metrics(todo_list);

        // Generate suggestions
        let suggestions = self.generate_suggestions(&issues, &metrics);

        // Determine overall validity
        let is_valid = !issues
            .iter()
            .any(|issue| issue.severity == IssueSeverity::Error);

        TodoValidationResult {
            is_valid,
            issues,
            metrics,
            suggestions,
        }
    }

    /// Validate individual todo
    fn validate_todo(&self, todo: &Todo, issues: &mut Vec<ValidationIssue>) {
        // Check actionability
        if !todo.is_actionable() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Actionability,
                todo_id: Some(todo.id.clone()),
                message: format!(
                    "Todo '{}' is not actionable - should start with action verb",
                    todo.content
                ),
                suggestion: Some(
                    "Start with verbs like 'implement', 'create', 'add', 'fix', etc.".to_string(),
                ),
            });
        }

        // Check content length
        let min_chars = self.config.min_task_detail_chars.unwrap_or(10);
        let max_chars = self.config.max_task_detail_chars.unwrap_or(100);

        if todo.content.len() < min_chars {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Completeness,
                todo_id: Some(todo.id.clone()),
                message: format!(
                    "Todo content too short: {} chars (min {})",
                    todo.content.len(),
                    min_chars
                ),
                suggestion: Some(
                    "Add more specific details about what needs to be done".to_string(),
                ),
            });
        }

        if todo.content.len() > max_chars {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Completeness,
                todo_id: Some(todo.id.clone()),
                message: format!(
                    "Todo content too long: {} chars (max {})",
                    todo.content.len(),
                    max_chars
                ),
                suggestion: Some("Break this into smaller, more focused tasks".to_string()),
            });
        }

        // Check complexity
        if let Some(max_complexity) = self.config.max_complexity_per_task {
            let complexity = todo.complexity_score();
            if complexity > max_complexity {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Complexity,
                    todo_id: Some(todo.id.clone()),
                    message: format!(
                        "Todo complexity {} exceeds maximum {}",
                        complexity, max_complexity
                    ),
                    suggestion: Some("Break this complex task into simpler subtasks".to_string()),
                });
            }
        }

        // Check time estimates
        if self.config.require_time_estimates && todo.estimated_hours.is_none() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::TimeEstimate,
                todo_id: Some(todo.id.clone()),
                message: "Todo missing time estimate".to_string(),
                suggestion: Some(
                    "Add estimated_hours field with realistic time estimate".to_string(),
                ),
            });
        }

        if let Some(hours) = todo.estimated_hours {
            let min_hours = self.config.min_estimated_hours.unwrap_or(0.5);
            let max_hours = self.config.max_estimated_hours.unwrap_or(40.0);

            if hours < min_hours {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    category: IssueCategory::TimeEstimate,
                    todo_id: Some(todo.id.clone()),
                    message: format!(
                        "Time estimate {:.1}h seems too low (min {:.1}h)",
                        hours, min_hours
                    ),
                    suggestion: Some(
                        "Consider if this task really needs so little time".to_string(),
                    ),
                });
            }

            if hours > max_hours {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::TimeEstimate,
                    todo_id: Some(todo.id.clone()),
                    message: format!(
                        "Time estimate {:.1}h exceeds maximum {:.1}h",
                        hours, max_hours
                    ),
                    suggestion: Some("Break this large task into smaller chunks".to_string()),
                });
            }
        }

        // Check for generic or vague language
        if self.config.require_specific_actions {
            let generic_words = [
                "thing",
                "stuff",
                "item",
                "something",
                "fix issues",
                "handle",
            ];
            let lower_content = todo.content.to_lowercase();

            for word in &generic_words {
                if lower_content.contains(word) {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        category: IssueCategory::Completeness,
                        todo_id: Some(todo.id.clone()),
                        message: format!("Todo contains generic language: '{}'", word),
                        suggestion: Some(
                            "Be more specific about what needs to be done".to_string(),
                        ),
                    });
                    break;
                }
            }
        }
    }

    /// Validate overall structure
    fn validate_structure(&self, todo_list: &TodoList, issues: &mut Vec<ValidationIssue>) {
        let count = todo_list.todos.len();

        // Check todo count limits
        if let Some(max_todos) = self.config.max_todos_per_batch {
            if count > max_todos {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Structure,
                    todo_id: None,
                    message: format!("Todo count {} exceeds maximum {}", count, max_todos),
                    suggestion: Some("Split into multiple smaller todo lists".to_string()),
                });
            }
        }

        if count == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Structure,
                todo_id: None,
                message: "Todo list is empty".to_string(),
                suggestion: Some("Add at least one todo item".to_string()),
            });
        }

        // Check for duplicate IDs
        let mut seen_ids = HashSet::new();
        for todo in &todo_list.todos {
            if !seen_ids.insert(todo.id.clone()) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Structure,
                    todo_id: Some(todo.id.clone()),
                    message: format!("Duplicate todo ID: {}", todo.id),
                    suggestion: Some("Ensure all todo IDs are unique".to_string()),
                });
            }
        }

        // Check for duplicate content
        let mut seen_content = HashMap::new();
        for todo in &todo_list.todos {
            let normalized = todo.content.trim().to_lowercase();
            if let Some(other_id) = seen_content.insert(normalized.clone(), todo.id.clone()) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    category: IssueCategory::Structure,
                    todo_id: Some(todo.id.clone()),
                    message: format!("Duplicate todo content with ID: {}", other_id),
                    suggestion: Some(
                        "Make todo descriptions more specific to avoid duplicates".to_string(),
                    ),
                });
            }
        }
    }

    /// Validate dependencies
    fn validate_dependencies(&self, todo_list: &TodoList, issues: &mut Vec<ValidationIssue>) {
        if !self.config.require_dependency_graph {
            return;
        }

        let todo_ids: HashSet<String> = todo_list.todos.iter().map(|t| t.id.clone()).collect();

        // Check for invalid dependency references
        for todo in &todo_list.todos {
            for dep_id in &todo.dependencies {
                if !todo_ids.contains(dep_id) {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Error,
                        category: IssueCategory::Dependencies,
                        todo_id: Some(todo.id.clone()),
                        message: format!("Dependency '{}' not found", dep_id),
                        suggestion: Some(
                            "Remove invalid dependency or add missing todo".to_string(),
                        ),
                    });
                }
            }
        }

        // Check for circular dependencies
        if self.config.prevent_circular_dependencies {
            if let Err(cycle) = todo_list.validate_dependencies() {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Dependencies,
                    todo_id: None,
                    message: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                    suggestion: Some(
                        "Remove circular dependencies by reordering tasks".to_string(),
                    ),
                });
            }
        }

        // Check for self-dependencies
        for todo in &todo_list.todos {
            if todo.dependencies.contains(&todo.id) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    category: IssueCategory::Dependencies,
                    todo_id: Some(todo.id.clone()),
                    message: "Todo depends on itself".to_string(),
                    suggestion: Some("Remove self-dependency".to_string()),
                });
            }
        }
    }

    /// Calculate quality metrics
    fn calculate_metrics(&self, todo_list: &TodoList) -> TodoMetrics {
        let total_count = todo_list.todos.len();
        let mut actionable_count = 0;
        let mut proper_length_count = 0;
        let mut estimated_count = 0;
        let mut reasonable_complexity_count = 0;
        let mut total_complexity = 0u32;
        let mut total_length = 0usize;
        let mut total_estimated_hours = 0.0f32;

        let min_chars = self.config.min_task_detail_chars.unwrap_or(10);
        let max_chars = self.config.max_task_detail_chars.unwrap_or(100);
        let max_complexity = self.config.max_complexity_per_task.unwrap_or(8);

        for todo in &todo_list.todos {
            if todo.is_actionable() {
                actionable_count += 1;
            }

            if todo.has_valid_length(min_chars, max_chars) {
                proper_length_count += 1;
            }

            if todo.estimated_hours.is_some() {
                estimated_count += 1;
                total_estimated_hours += todo.estimated_hours.unwrap_or(0.0);
            }

            let complexity = todo.complexity_score();
            total_complexity += complexity as u32;

            if complexity <= max_complexity {
                reasonable_complexity_count += 1;
            }

            total_length += todo.content.len();
        }

        let avg_complexity = if total_count > 0 {
            total_complexity as f32 / total_count as f32
        } else {
            0.0
        };

        let avg_task_length = if total_count > 0 {
            total_length as f32 / total_count as f32
        } else {
            0.0
        };

        // Calculate dependency metrics
        let dependency_metrics = self.calculate_dependency_metrics(todo_list);

        TodoMetrics {
            total_count,
            actionable_count,
            proper_length_count,
            estimated_count,
            reasonable_complexity_count,
            avg_complexity,
            avg_task_length,
            total_estimated_hours,
            dependency_metrics,
        }
    }

    /// Calculate dependency graph metrics
    fn calculate_dependency_metrics(&self, todo_list: &TodoList) -> DependencyMetrics {
        let mut todos_with_dependencies = 0;
        let mut total_dependencies = 0;

        for todo in &todo_list.todos {
            if !todo.dependencies.is_empty() {
                todos_with_dependencies += 1;
                total_dependencies += todo.dependencies.len();
            }
        }

        let has_cycles = todo_list.validate_dependencies().is_err();

        // Only calculate depth if no cycles (to avoid infinite recursion)
        let (max_depth, critical_path_length) = if has_cycles {
            (0, 0)
        } else {
            let depth = self.calculate_max_dependency_depth(todo_list);
            (depth, depth)
        };

        DependencyMetrics {
            todos_with_dependencies,
            total_dependencies,
            max_depth,
            has_cycles,
            critical_path_length,
        }
    }

    /// Calculate maximum dependency depth
    fn calculate_max_dependency_depth(&self, todo_list: &TodoList) -> usize {
        use std::collections::HashMap;

        let mut depth_cache: HashMap<String, usize> = HashMap::new();
        let mut max_depth = 0;

        for todo in &todo_list.todos {
            let depth = self.calculate_todo_depth(&todo.id, todo_list, &mut depth_cache);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    /// Calculate depth for a specific todo (recursive with memoization)
    fn calculate_todo_depth(
        &self,
        todo_id: &str,
        todo_list: &TodoList,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        // Check if already calculated (also handles cycles)
        if let Some(&cached_depth) = cache.get(todo_id) {
            return cached_depth;
        }

        // Mark as visited with depth 0 to detect cycles
        cache.insert(todo_id.to_string(), 0);

        let todo = todo_list.todos.iter().find(|t| t.id == todo_id);
        if let Some(todo) = todo {
            if todo.dependencies.is_empty() {
                cache.insert(todo_id.to_string(), 1);
                return 1;
            }

            let mut max_dep_depth = 0;
            for dep_id in &todo.dependencies {
                // If we find a cycle (depth is 0), skip this dependency
                if cache.get(dep_id) == Some(&0) {
                    continue;
                }
                let dep_depth = self.calculate_todo_depth(dep_id, todo_list, cache);
                max_dep_depth = max_dep_depth.max(dep_depth);
            }

            let depth = max_dep_depth + 1;
            cache.insert(todo_id.to_string(), depth);
            depth
        } else {
            0
        }
    }

    /// Generate improvement suggestions
    fn generate_suggestions(
        &self,
        _issues: &[ValidationIssue],
        metrics: &TodoMetrics,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Actionability suggestions
        if metrics.actionable_count < metrics.total_count {
            let non_actionable = metrics.total_count - metrics.actionable_count;
            suggestions.push(format!(
                "Make {} todos more actionable by starting with action verbs (implement, create, add, etc.)",
                non_actionable
            ));
        }

        // Complexity suggestions
        if metrics.reasonable_complexity_count < metrics.total_count {
            suggestions.push(
                "Break down complex tasks (complexity > 8) into smaller, focused subtasks"
                    .to_string(),
            );
        }

        // Time estimate suggestions
        if self.config.require_time_estimates && metrics.estimated_count < metrics.total_count {
            let missing_estimates = metrics.total_count - metrics.estimated_count;
            suggestions.push(format!(
                "Add time estimates to {} todos for better project planning",
                missing_estimates
            ));
        }

        // Dependency suggestions
        if metrics.dependency_metrics.has_cycles {
            suggestions
                .push("Remove circular dependencies to enable proper task ordering".to_string());
        }

        if metrics.dependency_metrics.todos_with_dependencies == 0 && metrics.total_count > 1 {
            suggestions.push(
                "Consider adding dependencies between related tasks for better sequencing"
                    .to_string(),
            );
        }

        // Overall quality suggestions
        let quality_score = self.calculate_quality_score(metrics);
        if quality_score < 0.8 {
            suggestions.push("Overall todo list quality could be improved - focus on specific, actionable tasks with realistic estimates".to_string());
        }

        suggestions
    }

    /// Calculate overall quality score (0.0 to 1.0)
    fn calculate_quality_score(&self, metrics: &TodoMetrics) -> f32 {
        if metrics.total_count == 0 {
            return 0.0;
        }

        let actionability_score = metrics.actionable_count as f32 / metrics.total_count as f32;
        let length_score = metrics.proper_length_count as f32 / metrics.total_count as f32;
        let complexity_score =
            metrics.reasonable_complexity_count as f32 / metrics.total_count as f32;

        let estimate_score = if self.config.require_time_estimates {
            metrics.estimated_count as f32 / metrics.total_count as f32
        } else {
            1.0 // Don't penalize if estimates not required
        };

        let dependency_score = if metrics.dependency_metrics.has_cycles {
            0.0 // Circular dependencies are critical errors
        } else {
            1.0
        };

        // Weighted average
        actionability_score * 0.3
            + length_score * 0.2
            + complexity_score * 0.2
            + estimate_score * 0.2
            + dependency_score * 0.1
    }
}

impl Default for TodoValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Error => write!(f, "ERROR"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Info => write!(f, "INFO"),
        }
    }
}

impl std::fmt::Display for IssueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueCategory::Actionability => write!(f, "Actionability"),
            IssueCategory::Completeness => write!(f, "Completeness"),
            IssueCategory::Complexity => write!(f, "Complexity"),
            IssueCategory::TimeEstimate => write!(f, "Time Estimate"),
            IssueCategory::Dependencies => write!(f, "Dependencies"),
            IssueCategory::Structure => write!(f, "Structure"),
            IssueCategory::QualityGate => write!(f, "Quality Gate"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test imports handled by parent module

    #[test]
    fn test_todo_validation_basic() {
        let validator = TodoValidator::new();

        let mut todo = Todo::new("Implement user authentication");
        todo.estimated_hours = Some(4.0);

        let mut issues = Vec::new();
        validator.validate_todo(&todo, &mut issues);

        // Should have no errors for a well-formed todo
        assert_eq!(
            issues
                .iter()
                .filter(|i| i.severity == IssueSeverity::Error)
                .count(),
            0
        );
    }

    #[test]
    fn test_todo_validation_not_actionable() {
        let validator = TodoValidator::new();

        let todo = Todo::new("User authentication stuff");
        let mut issues = Vec::new();
        validator.validate_todo(&todo, &mut issues);

        assert!(issues
            .iter()
            .any(|i| i.category == IssueCategory::Actionability));
    }

    #[test]
    fn test_todo_list_validation() {
        let validator = TodoValidator::new();

        let mut todo_list = TodoList::new();
        let mut todo1 = Todo::new("Implement authentication system");
        todo1.id = "todo1".to_string();
        todo1.estimated_hours = Some(8.0);

        let mut todo2 = Todo::new("Create user interface");
        todo2.id = "todo2".to_string();
        todo2.estimated_hours = Some(6.0);
        todo2.dependencies = vec!["todo1".to_string()];

        todo_list.add_todo(todo1);
        todo_list.add_todo(todo2);

        let result = validator.validate_todo_list(&todo_list);

        assert!(result.is_valid);
        assert_eq!(result.metrics.total_count, 2);
        assert_eq!(result.metrics.actionable_count, 2);
        assert!(!result.metrics.dependency_metrics.has_cycles);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let validator = TodoValidator::new();

        let mut todo_list = TodoList::new();
        let mut todo1 = Todo::new("Task 1");
        todo1.id = "task1".to_string();
        todo1.dependencies = vec!["task2".to_string()];

        let mut todo2 = Todo::new("Task 2");
        todo2.id = "task2".to_string();
        todo2.dependencies = vec!["task1".to_string()];

        todo_list.add_todo(todo1);
        todo_list.add_todo(todo2);

        let result = validator.validate_todo_list(&todo_list);

        assert!(!result.is_valid);
        assert!(result
            .issues
            .iter()
            .any(|i| i.category == IssueCategory::Dependencies));
    }

    #[test]
    fn test_quality_metrics_calculation() {
        let validator = TodoValidator::new();

        let mut todo_list = TodoList::new();

        // Good todo
        let mut good_todo = Todo::new("Implement user login endpoint");
        good_todo.estimated_hours = Some(3.0);
        todo_list.add_todo(good_todo);

        // Poor todo (still actionable but vague)
        let poor_todo = Todo::new("stuff to handle");
        todo_list.add_todo(poor_todo);

        let result = validator.validate_todo_list(&todo_list);

        assert_eq!(result.metrics.total_count, 2);
        assert_eq!(result.metrics.actionable_count, 1);
        assert_eq!(result.metrics.estimated_count, 1);
        assert!((result.metrics.total_estimated_hours - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_suggestion_generation() {
        let validator = TodoValidator::new();

        let mut todo_list = TodoList::new();
        let todo = Todo::new("stuff to do"); // Non-actionable, no estimate
        todo_list.add_todo(todo);

        let result = validator.validate_todo_list(&todo_list);

        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions.iter().any(|s| s.contains("actionable")));
    }
}
