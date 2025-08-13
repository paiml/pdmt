//! Todo List Generation Example
//!
//! This example demonstrates the core functionality of PDMT for generating
//! deterministic todo lists with quality enforcement and granular detail.
//!
//! Run with: cargo run --example todo_generation

use clap::{Parser, ValueEnum};
use console::{style, Term};
use dialoguer::{Confirm, Input, Select};
use pdmt::models::todo::{Todo, TodoGranularity, TodoInput, TodoList, TodoPriority};
// JSON and collections used for structured data handling

#[derive(Parser, Debug)]
#[command(name = "todo-generation")]
#[command(about = "Generate deterministic todo lists with quality enforcement")]
struct Args {
    /// Project name
    #[arg(short, long)]
    project: Option<String>,

    /// Requirements (can be specified multiple times)
    #[arg(short, long, action = clap::ArgAction::Append)]
    requirement: Vec<String>,

    /// Task granularity level
    #[arg(short, long, value_enum, default_value = "high")]
    granularity: GranularityArg,

    /// Maximum number of todos to generate
    #[arg(short, long, default_value = "20")]
    max_todos: usize,

    /// Include time estimates
    #[arg(short = 'e', long, default_value = "true")]
    estimates: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "yaml")]
    format: FormatArg,

    /// Interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Quality mode (strict validation)
    #[arg(short, long)]
    quality: bool,

    /// Show detailed output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum GranularityArg {
    High,
    Medium,
    Low,
}

#[derive(Clone, Debug, ValueEnum)]
enum FormatArg {
    Yaml,
    Json,
    Markdown,
    Text,
}

impl From<GranularityArg> for TodoGranularity {
    fn from(arg: GranularityArg) -> Self {
        match arg {
            GranularityArg::High => TodoGranularity::High,
            GranularityArg::Medium => TodoGranularity::Medium,
            GranularityArg::Low => TodoGranularity::Low,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let term = Term::stdout();

    // Print header
    term.write_line(&format!(
        "{}",
        style("🚀 PDMT Todo Generation Example").bold().cyan()
    ))?;
    term.write_line(&format!(
        "{}",
        style("Generate deterministic todo lists with quality enforcement").dim()
    ))?;
    term.write_line("")?;

    // Get input data
    let input = if args.interactive {
        get_interactive_input(&args).await?
    } else {
        get_args_input(&args)?
    };

    if args.verbose {
        term.write_line(&format!("{}", style("📋 Input Configuration:").bold()))?;
        term.write_line(&format!("  Project: {}", input.project_name))?;
        term.write_line(&format!(
            "  Requirements: {} items",
            input.requirements.len()
        ))?;
        term.write_line(&format!("  Granularity: {:?}", input.granularity))?;
        term.write_line(&format!("  Max todos: {:?}", input.max_todos))?;
        term.write_line(&format!("  Include estimates: {}", input.include_estimates))?;
        term.write_line("")?;
    }

    // Generate todo list
    term.write_line(&format!(
        "{}",
        style("🔄 Generating deterministic todo list...").yellow()
    ))?;

    let start_time = std::time::Instant::now();
    let todo_list = generate_deterministic_todos(&input, args.quality, args.verbose).await?;
    let generation_time = start_time.elapsed();

    term.write_line(&format!(
        "{} Generated {} todos in {:?}",
        style("✅").green(),
        todo_list.todos.len(),
        generation_time
    ))?;
    term.write_line("")?;

    // Display results
    match args.format {
        FormatArg::Yaml => {
            term.write_line(&format!("{}", style("📄 Generated YAML:").bold()))?;
            let yaml = serde_yaml::to_string(&todo_list)?;
            term.write_line(&yaml)?;
        }
        FormatArg::Json => {
            term.write_line(&format!("{}", style("📄 Generated JSON:").bold()))?;
            let json = serde_json::to_string_pretty(&todo_list)?;
            term.write_line(&json)?;
        }
        FormatArg::Markdown => {
            term.write_line(&format!("{}", style("📄 Generated Markdown:").bold()))?;
            let markdown = format_as_markdown(&todo_list)?;
            term.write_line(&markdown)?;
        }
        FormatArg::Text => {
            term.write_line(&format!("{}", style("📄 Generated Text:").bold()))?;
            let text = format_as_text(&todo_list)?;
            term.write_line(&text)?;
        }
    }

    // Show statistics
    if args.verbose {
        show_statistics(&todo_list, &term)?;
    }

    // Quality validation summary
    if args.quality {
        show_quality_summary(&todo_list, &term)?;
    }

    Ok(())
}

async fn get_interactive_input(args: &Args) -> Result<TodoInput, Box<dyn std::error::Error>> {
    let term = Term::stdout();

    term.write_line(&format!(
        "{}",
        style("🎯 Interactive Todo Generation").bold()
    ))?;
    term.write_line("")?;

    // Get project name
    let project_name: String = Input::new()
        .with_prompt("Project name")
        .default(
            args.project
                .clone()
                .unwrap_or_else(|| "My Project".to_string()),
        )
        .interact()?;

    // Get requirements
    let mut requirements = Vec::new();
    if args.requirement.is_empty() {
        term.write_line("Enter project requirements (empty line to finish):")?;
        loop {
            let requirement: String = Input::new()
                .with_prompt(&format!("Requirement #{}", requirements.len() + 1))
                .allow_empty(true)
                .interact()?;

            if requirement.trim().is_empty() {
                break;
            }
            requirements.push(requirement);
        }
    } else {
        requirements = args.requirement.clone();
    }

    // Get granularity
    let granularity_options = [
        "High (many small tasks)",
        "Medium (balanced)",
        "Low (fewer large tasks)",
    ];
    let granularity_selection = Select::new()
        .with_prompt("Task granularity")
        .items(&granularity_options)
        .default(0)
        .interact()?;

    let granularity = match granularity_selection {
        0 => TodoGranularity::High,
        1 => TodoGranularity::Medium,
        2 => TodoGranularity::Low,
        _ => TodoGranularity::High,
    };

    // Get additional options
    let include_estimates = Confirm::new()
        .with_prompt("Include time estimates?")
        .default(args.estimates)
        .interact()?;

    let max_todos = if requirements.len() > 3 {
        Some(args.max_todos)
    } else {
        Some(args.max_todos / 2)
    };

    Ok(TodoInput {
        project_name,
        requirements,
        granularity,
        project_context: None,
        quality_config: None,
        max_todos,
        include_estimates,
        default_priority: Some(TodoPriority::Medium),
    })
}

fn get_args_input(args: &Args) -> Result<TodoInput, Box<dyn std::error::Error>> {
    let project_name = args
        .project
        .clone()
        .unwrap_or_else(|| "Sample Project".to_string());

    let requirements = if args.requirement.is_empty() {
        vec![
            "Create user authentication system".to_string(),
            "Implement REST API endpoints".to_string(),
            "Add database integration".to_string(),
            "Write comprehensive tests".to_string(),
            "Setup CI/CD pipeline".to_string(),
        ]
    } else {
        args.requirement.clone()
    };

    Ok(TodoInput {
        project_name,
        requirements,
        granularity: args.granularity.clone().into(),
        project_context: None,
        quality_config: None,
        max_todos: Some(args.max_todos),
        include_estimates: args.estimates,
        default_priority: Some(TodoPriority::Medium),
    })
}

async fn generate_deterministic_todos(
    input: &TodoInput,
    quality_mode: bool,
    verbose: bool,
) -> Result<TodoList, Box<dyn std::error::Error>> {
    let term = Term::stdout();

    // Create deterministic todo list based on input
    let mut todo_list = TodoList::new();

    // Generate todos based on requirements and granularity
    let todos_per_requirement = match input.granularity {
        TodoGranularity::High => 4,
        TodoGranularity::Medium => 2,
        TodoGranularity::Low => 1,
    };

    let max_todos = input.max_todos.unwrap_or(20);
    let mut todo_count = 0;

    if verbose {
        term.write_line(&format!(
            "  Generating {} todos per requirement",
            todos_per_requirement
        ))?;
    }

    for (req_idx, requirement) in input.requirements.iter().enumerate() {
        if todo_count >= max_todos {
            break;
        }

        let todos_for_req = generate_todos_for_requirement(
            requirement,
            req_idx,
            todos_per_requirement,
            &input.granularity,
            input.include_estimates,
        )?;

        for todo in todos_for_req {
            if todo_count >= max_todos {
                break;
            }
            todo_list.add_todo(todo);
            todo_count += 1;
        }
    }

    // Apply quality gates if requested
    if quality_mode {
        if verbose {
            term.write_line("  Applying quality gates...")?;
        }
        apply_quality_gates(&mut todo_list)?;
    }

    todo_list.update_metadata();

    Ok(todo_list)
}

fn generate_todos_for_requirement(
    requirement: &str,
    req_idx: usize,
    count: usize,
    granularity: &TodoGranularity,
    include_estimates: bool,
) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let mut todos = Vec::new();

    // Define task templates based on granularity
    let task_templates = match granularity {
        TodoGranularity::High => get_high_granularity_templates(requirement),
        TodoGranularity::Medium => get_medium_granularity_templates(requirement),
        TodoGranularity::Low => get_low_granularity_templates(requirement),
    };

    for (idx, template) in task_templates.iter().take(count).enumerate() {
        let mut todo = Todo::new(template);
        todo.id = format!("todo_{}_{}", req_idx, idx);

        // Set deterministic priority based on position
        todo.priority = match idx {
            0 => TodoPriority::High,                  // First task is high priority
            n if n == count - 1 => TodoPriority::Low, // Last task is low priority
            _ => TodoPriority::Medium,                // Middle tasks are medium priority
        };

        // Add time estimates if requested
        if include_estimates {
            todo.estimated_hours = Some(estimate_hours_for_task(template));
        }

        // Add dependencies for sequential tasks
        if idx > 0 {
            todo.dependencies
                .push(format!("todo_{}_{}", req_idx, idx - 1));
        }

        // Add tags based on task content
        todo.tags = extract_tags_from_content(template);

        todos.push(todo);
    }

    Ok(todos)
}

fn get_high_granularity_templates(requirement: &str) -> Vec<String> {
    let req_lower = requirement.to_lowercase();

    if req_lower.contains("auth") {
        vec![
            "Design user authentication schema".to_string(),
            "Implement user registration endpoint".to_string(),
            "Create login/logout functionality".to_string(),
            "Add password hashing and validation".to_string(),
            "Implement JWT token management".to_string(),
            "Create user session handling".to_string(),
        ]
    } else if req_lower.contains("api") || req_lower.contains("endpoint") {
        vec![
            "Design API endpoint specifications".to_string(),
            "Implement CRUD operations for core entities".to_string(),
            "Add request validation middleware".to_string(),
            "Create API response formatting".to_string(),
            "Implement error handling for API routes".to_string(),
            "Add API rate limiting".to_string(),
        ]
    } else if req_lower.contains("database") {
        vec![
            "Design database schema and relationships".to_string(),
            "Create database migration scripts".to_string(),
            "Implement database connection pool".to_string(),
            "Add database query optimization".to_string(),
            "Create data access layer".to_string(),
            "Implement database backup strategy".to_string(),
        ]
    } else if req_lower.contains("test") {
        vec![
            "Create unit test framework setup".to_string(),
            "Write unit tests for core functions".to_string(),
            "Implement integration test suite".to_string(),
            "Add API endpoint testing".to_string(),
            "Create test data fixtures".to_string(),
            "Setup test coverage reporting".to_string(),
        ]
    } else {
        vec![
            format!("Analyze requirements for {}", requirement),
            format!("Design solution architecture for {}", requirement),
            format!("Implement core functionality for {}", requirement),
            format!("Add error handling for {}", requirement),
            format!("Create documentation for {}", requirement),
            format!("Test and validate {}", requirement),
        ]
    }
}

fn get_medium_granularity_templates(requirement: &str) -> Vec<String> {
    let req_lower = requirement.to_lowercase();

    if req_lower.contains("auth") {
        vec![
            "Implement user authentication system".to_string(),
            "Create authentication middleware and security".to_string(),
        ]
    } else if req_lower.contains("api") {
        vec![
            "Design and implement REST API endpoints".to_string(),
            "Add API validation and error handling".to_string(),
        ]
    } else if req_lower.contains("database") {
        vec![
            "Setup database schema and connections".to_string(),
            "Implement data access layer".to_string(),
        ]
    } else if req_lower.contains("test") {
        vec![
            "Create comprehensive test suite".to_string(),
            "Setup test automation and coverage".to_string(),
        ]
    } else {
        vec![
            format!("Implement {}", requirement),
            format!("Test and document {}", requirement),
        ]
    }
}

fn get_low_granularity_templates(requirement: &str) -> Vec<String> {
    vec![format!("Implement complete {}", requirement)]
}

fn estimate_hours_for_task(task: &str) -> f32 {
    let task_lower = task.to_lowercase();

    // Base estimate
    let mut hours: f32 = 2.0;

    // Adjust based on complexity keywords
    if task_lower.contains("design") || task_lower.contains("architect") {
        hours += 2.0;
    }
    if task_lower.contains("implement") || task_lower.contains("create") {
        hours += 3.0;
    }
    if task_lower.contains("test") || task_lower.contains("validate") {
        hours += 1.5;
    }
    if task_lower.contains("database") || task_lower.contains("schema") {
        hours += 2.0;
    }
    if task_lower.contains("api") || task_lower.contains("endpoint") {
        hours += 1.5;
    }
    if task_lower.contains("security") || task_lower.contains("auth") {
        hours += 3.0;
    }
    if task_lower.contains("integration") || task_lower.contains("middleware") {
        hours += 2.5;
    }

    // Cap at reasonable limits
    hours.max(0.5).min(16.0)
}

fn extract_tags_from_content(content: &str) -> Vec<String> {
    let content_lower = content.to_lowercase();
    let mut tags = Vec::new();

    // Technical area tags
    if content_lower.contains("auth") || content_lower.contains("login") {
        tags.push("authentication".to_string());
    }
    if content_lower.contains("api") || content_lower.contains("endpoint") {
        tags.push("api".to_string());
    }
    if content_lower.contains("database") || content_lower.contains("schema") {
        tags.push("database".to_string());
    }
    if content_lower.contains("test") || content_lower.contains("validate") {
        tags.push("testing".to_string());
    }
    if content_lower.contains("security") {
        tags.push("security".to_string());
    }
    if content_lower.contains("frontend") || content_lower.contains("ui") {
        tags.push("frontend".to_string());
    }
    if content_lower.contains("backend") {
        tags.push("backend".to_string());
    }

    // Task type tags
    if content_lower.starts_with("design") {
        tags.push("design".to_string());
    }
    if content_lower.starts_with("implement") || content_lower.starts_with("create") {
        tags.push("implementation".to_string());
    }
    if content_lower.starts_with("test") {
        tags.push("testing".to_string());
    }
    if content_lower.contains("document") {
        tags.push("documentation".to_string());
    }

    tags
}

fn apply_quality_gates(todo_list: &mut TodoList) -> Result<(), Box<dyn std::error::Error>> {
    for todo in &mut todo_list.todos {
        // Apply quality checks
        todo.quality_gates.actionability_check = todo.is_actionable();
        todo.quality_gates.completeness_check =
            todo.content.len() >= 10 && todo.content.len() <= 100;
        todo.quality_gates.complexity_check = todo.complexity_score() <= 8;
        todo.quality_gates.time_estimate_check = todo.estimated_hours.is_some();

        // Force quality improvements if needed
        if !todo.quality_gates.actionability_check {
            // Make task more actionable
            if !todo.content.starts_with("Implement") && !todo.content.starts_with("Create") {
                todo.content = format!("Implement {}", todo.content.to_lowercase());
            }
            todo.quality_gates.actionability_check = true;
        }

        if !todo.quality_gates.completeness_check && todo.content.len() < 10 {
            // Add more detail
            todo.content = format!("{} with proper error handling and validation", todo.content);
            todo.quality_gates.completeness_check = true;
        }

        if !todo.quality_gates.time_estimate_check {
            // Add default estimate
            todo.estimated_hours = Some(estimate_hours_for_task(&todo.content));
            todo.quality_gates.time_estimate_check = true;
        }
    }

    Ok(())
}

fn show_statistics(todo_list: &TodoList, term: &Term) -> Result<(), Box<dyn std::error::Error>> {
    term.write_line("")?;
    term.write_line(&format!("{}", style("📊 Todo List Statistics:").bold()))?;
    term.write_line(&format!(
        "  Total todos: {}",
        todo_list.metadata.total_count
    ))?;
    term.write_line(&format!(
        "  Total estimated hours: {:.1}",
        todo_list.metadata.total_estimated_hours
    ))?;
    term.write_line(&format!(
        "  Average hours per task: {:.1}",
        todo_list.metadata.avg_estimated_hours
    ))?;
    term.write_line(&format!(
        "  Dependency graph valid: {}",
        if todo_list.metadata.dependency_graph_valid {
            "✅"
        } else {
            "❌"
        }
    ))?;

    // Status breakdown
    term.write_line("  Status breakdown:")?;
    for (status, count) in &todo_list.metadata.status_counts {
        term.write_line(&format!("    {}: {}", status, count))?;
    }

    // Priority breakdown
    term.write_line("  Priority breakdown:")?;
    for (priority, count) in &todo_list.metadata.priority_counts {
        term.write_line(&format!("    {}: {}", priority, count))?;
    }

    Ok(())
}

fn show_quality_summary(
    todo_list: &TodoList,
    term: &Term,
) -> Result<(), Box<dyn std::error::Error>> {
    term.write_line("")?;
    term.write_line(&format!(
        "{}",
        style("✅ Quality Gate Summary:").bold().green()
    ))?;

    let mut actionable_count = 0;
    let mut complete_count = 0;
    let mut complexity_count = 0;
    let mut estimate_count = 0;

    for todo in &todo_list.todos {
        if todo.quality_gates.actionability_check {
            actionable_count += 1;
        }
        if todo.quality_gates.completeness_check {
            complete_count += 1;
        }
        if todo.quality_gates.complexity_check {
            complexity_count += 1;
        }
        if todo.quality_gates.time_estimate_check {
            estimate_count += 1;
        }
    }

    let total = todo_list.todos.len();

    term.write_line(&format!(
        "  Actionability: {}/{} ({:.1}%)",
        actionable_count,
        total,
        (actionable_count as f32 / total as f32) * 100.0
    ))?;
    term.write_line(&format!(
        "  Completeness: {}/{} ({:.1}%)",
        complete_count,
        total,
        (complete_count as f32 / total as f32) * 100.0
    ))?;
    term.write_line(&format!(
        "  Complexity: {}/{} ({:.1}%)",
        complexity_count,
        total,
        (complexity_count as f32 / total as f32) * 100.0
    ))?;
    term.write_line(&format!(
        "  Time estimates: {}/{} ({:.1}%)",
        estimate_count,
        total,
        (estimate_count as f32 / total as f32) * 100.0
    ))?;

    Ok(())
}

fn format_as_markdown(todo_list: &TodoList) -> Result<String, Box<dyn std::error::Error>> {
    let mut markdown = String::new();

    markdown.push_str("# Todo List\n\n");

    // Add metadata
    markdown.push_str("## Summary\n\n");
    markdown.push_str(&format!(
        "- **Total todos**: {}\n",
        todo_list.metadata.total_count
    ));
    markdown.push_str(&format!(
        "- **Total estimated hours**: {:.1}\n",
        todo_list.metadata.total_estimated_hours
    ));
    markdown.push_str(&format!(
        "- **Average hours per task**: {:.1}\n",
        todo_list.metadata.avg_estimated_hours
    ));
    markdown.push_str("\n");

    // Add todos
    markdown.push_str("## Tasks\n\n");
    for (idx, todo) in todo_list.todos.iter().enumerate() {
        markdown.push_str(&format!("### {}. {}\n\n", idx + 1, todo.content));
        markdown.push_str(&format!("- **ID**: `{}`\n", todo.id));
        markdown.push_str(&format!("- **Status**: {}\n", todo.status));
        markdown.push_str(&format!("- **Priority**: {}\n", todo.priority));

        if let Some(hours) = todo.estimated_hours {
            markdown.push_str(&format!("- **Estimated hours**: {:.1}\n", hours));
        }

        if !todo.dependencies.is_empty() {
            markdown.push_str(&format!(
                "- **Dependencies**: {}\n",
                todo.dependencies.join(", ")
            ));
        }

        if !todo.tags.is_empty() {
            markdown.push_str(&format!("- **Tags**: {}\n", todo.tags.join(", ")));
        }

        markdown.push_str("\n");
    }

    Ok(markdown)
}

fn format_as_text(todo_list: &TodoList) -> Result<String, Box<dyn std::error::Error>> {
    let mut text = String::new();

    text.push_str("TODO LIST\n");
    text.push_str("=========\n\n");

    text.push_str(&format!(
        "Total todos: {}\n",
        todo_list.metadata.total_count
    ));
    text.push_str(&format!(
        "Total estimated hours: {:.1}\n",
        todo_list.metadata.total_estimated_hours
    ));
    text.push_str("\n");

    for (idx, todo) in todo_list.todos.iter().enumerate() {
        text.push_str(&format!(
            "{}. {} [{}]\n",
            idx + 1,
            todo.content,
            todo.status
        ));
        text.push_str(&format!("   ID: {}\n", todo.id));
        text.push_str(&format!("   Priority: {}\n", todo.priority));

        if let Some(hours) = todo.estimated_hours {
            text.push_str(&format!("   Estimated: {:.1}h\n", hours));
        }

        if !todo.dependencies.is_empty() {
            text.push_str(&format!(
                "   Depends on: {}\n",
                todo.dependencies.join(", ")
            ));
        }

        text.push_str("\n");
    }

    Ok(text)
}
