# PDMT - Pragmatic Deterministic MCP Templating

[![Crates.io](https://img.shields.io/crates/v/pdmt.svg)](https://crates.io/crates/pdmt)
[![Documentation](https://docs.rs/pdmt/badge.svg)](https://docs.rs/pdmt)
[![Build Status](https://github.com/paiml/pdmt/workflows/CI/badge.svg)](https://github.com/paiml/pdmt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, deterministic templating library for Model Context Protocol (MCP) applications, designed for generating consistent, validated todo lists and structured content with quality enforcement.

## 🚀 Features

- **🎯 Deterministic Generation**: 0.0 temperature templating ensures reproducible outputs
- **📋 Todo Validation**: Comprehensive validation with actionability checks, complexity scoring, and time estimates
- **🔄 MCP Integration**: Native support for Model Context Protocol via PMCP SDK
- **🛡️ Quality Gates**: PMAT (Paiml MCP Agent Toolkit) integration for quality enforcement
- **🧪 Extensive Testing**: 81%+ test coverage with property testing, fuzz testing, and edge case coverage
- **⚡ High Performance**: Optimized Handlebars engine with caching and validation
- **📦 Multiple Formats**: Support for YAML, JSON, Markdown, and plain text output
- **🔍 Dependency Analysis**: Circular dependency detection and critical path calculation

## 📚 Quick Start

Add PDMT to your `Cargo.toml`:

```toml
[dependencies]
pdmt = "1.0.0"

# Optional features
pdmt = { version = "1.0.0", features = ["quality-proxy", "mcp-tools", "todo-validation"] }
```

### Basic Usage

```rust
use pdmt::{TemplateEngine, models::todo::TodoInput};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create template engine
    let mut engine = TemplateEngine::new();
    engine.load_builtin_templates().await?;
    
    // Create todo input
    let input = TodoInput {
        project_name: "My Project".to_string(),
        requirements: vec![
            "Implement user authentication".to_string(),
            "Create REST API endpoints".to_string(),
            "Add comprehensive tests".to_string(),
        ],
        granularity: pdmt::models::todo::TodoGranularity::High,
        include_estimates: true,
        max_todos: Some(10),
        ..Default::default()
    };
    
    // Generate deterministic todo list
    let result = engine.generate("todo_list", input).await?;
    
    println!("Generated todos:\n{}", result.content);
    println!("Template used: {}", result.template_id);
    
    Ok(())
}
```

### Advanced Features

#### Todo Validation

```rust
use pdmt::{validators::todo::TodoValidator, models::todo::TodoList};

let validator = TodoValidator::new();
let mut todo_list = TodoList::new();

// Add todos...
todo_list.add_todo(Todo::new("Implement authentication system"));

let validation_result = validator.validate_todo_list(&todo_list);

if validation_result.is_valid {
    println!("✅ All todos are valid!");
    println!("Quality score: {:.2}", validation_result.quality_score);
} else {
    println!("❌ Validation issues found:");
    for issue in validation_result.issues {
        println!("  - {}: {}", issue.category, issue.message);
    }
}
```

## 🏗️ Architecture

PDMT is built with a modular architecture:

- **Template Engine**: Handlebars-based deterministic generation
- **Content Models**: Todo lists, generated content, and metadata
- **Quality Proxy**: PMAT integration for quality enforcement  
- **MCP Integration**: Native Model Context Protocol support
- **Validators**: Comprehensive validation for todos and content

## 🎯 Feature Flags

```toml
[dependencies]
pdmt = { 
    version = "1.0.0", 
    features = [
        "quality-proxy",     # PMAT quality enforcement
        "mcp-tools",        # MCP/PMCP integration  
        "todo-validation",  # Advanced todo validation
        "property-tests",   # Property testing support
    ]
}
```

## 📝 YAML Template System

PDMT uses a powerful YAML-based template system for deterministic content generation:

### Template Structure

```yaml
# Basic template structure
id: todo_list
version: "1.0.0"
extends: base  # Optional inheritance

metadata:
  provider: "deterministic"
  description: "Generate deterministic todo lists"
  parameters:
    temperature: 0.0  # Ensures deterministic output
    
input_schema:
  type: object
  required: ["project_name", "requirements"]
  properties:
    project_name:
      type: string
      description: "Name of the project"
    requirements:
      type: array
      items:
        type: string
      description: "List of requirements to convert to tasks"
    granularity:
      type: string
      enum: ["low", "medium", "high"]
      default: "high"
      
validation:
  deterministic_only: true
  quality_gates:
    max_complexity_per_task: 8
    require_time_estimates: true
    require_specific_actions: true
    min_task_detail_chars: 10
    max_task_detail_chars: 100
    
prompt_template: |
  Generate a deterministic todo list for "{{project_name}}".
  Requirements:
  {{#each requirements}}
  - {{this}}
  {{/each}}
```

### Built-in Templates

PDMT provides several built-in templates:

| Template ID | Description | Use Case |
|------------|-------------|----------|
| `todo_list` | Deterministic todo generation | Project planning, task breakdown |
| `project_scaffold` | Project structure generation | New project setup |
| `base` | Base template for inheritance | Template extension |

### Custom Templates

Create custom templates by extending the base template:

```yaml
# custom_template.yml
id: custom_engineering_todos
version: "1.0.0"
extends: todo_list

metadata:
  description: "Engineering-specific todo generation"
  
validation:
  quality_gates:
    require_test_specifications: true
    require_documentation: true
    enforce_dependency_tracking: true
    
output_schema:
  format: yaml
  structure: |
    todos:
      - id: string
        content: string
        estimated_hours: number
        test_requirements: [string]
        documentation_requirements: [string]
```

### Loading Templates

```rust
use pdmt::TemplateEngine;

let mut engine = TemplateEngine::new();

// Load built-in templates
engine.load_builtin_templates().await?;

// Load custom template from file
engine.load_template_file("templates/custom_template.yml").await?;

// Load template from string
let yaml_content = std::fs::read_to_string("template.yml")?;
engine.load_template(&yaml_content)?;
```

## 📖 Examples

The `examples/` directory contains comprehensive examples demonstrating PDMT's deterministic templating capabilities:

### 📋 Todo Generation
Generate deterministic, quality-enforced todo lists for project planning:

```bash
cargo run --example todo_generation --features="full" -- \
  --project "My Project" \
  --requirement "Implement feature X" \
  --requirement "Add tests for Y" \
  --granularity high \
  --max-todos 10 \
  --format yaml
```

### 📄 Resume Builder
Create professional resumes with consistent formatting:

```bash
cargo run --example resume_builder --features="full" -- \
  --name "Jane Doe" \
  --title "Senior Software Engineer" \
  --email "jane@example.com" \
  --format markdown

# Interactive mode for detailed input
cargo run --example resume_builder --features="full" -- --interactive
```

### 📚 README Builder
Generate well-structured README files with standardized sections:

```bash
cargo run --example readme_builder --features="full" -- \
  --name "my-awesome-project" \
  --description "A powerful Rust library" \
  --language rust \
  --github-user myusername \
  --badges \
  --output README.md

# Interactive mode for guided setup
cargo run --example readme_builder --features="full" -- --interactive
```

Each example demonstrates:
- **Deterministic output** - Same inputs always produce identical results
- **YAML templating** - Structured templates for consistent formatting
- **Quality enforcement** - Validation and best practices built-in
- **Multiple output formats** - Support for various file formats

## 🛡️ Quality Enforcement

PDMT integrates with PAIML's quality gate system for comprehensive validation:

### Quality Gates

| Quality Check | Description | Threshold |
|--------------|-------------|-----------|
| **Actionability** | Tasks must start with action verbs | 100% required |
| **Complexity** | Cyclomatic complexity limits | Max score: 8 |
| **Time Estimates** | Realistic effort estimation | 0.5-40 hours |
| **Length Validation** | Task description constraints | 10-100 chars |
| **Dependency Tracking** | Circular dependency detection | Zero cycles |
| **SATD Detection** | No TODO/FIXME/HACK comments | Zero tolerance |

### Validation Example

```rust
use pdmt::{
    validators::todo::{TodoValidator, TodoQualityConfig},
    models::todo::{TodoList, Todo}
};

// Configure quality requirements
let config = TodoQualityConfig {
    max_complexity_per_task: Some(8),
    require_time_estimates: true,
    require_specific_actions: true,
    min_task_detail_chars: Some(10),
    max_task_detail_chars: Some(100),
    ..Default::default()
};

let validator = TodoValidator::with_config(config);
let todo_list = TodoList::new();

// Validate todos
let result = validator.validate_todo_list(&todo_list);

// Check quality metrics
println!("Actionable tasks: {}/{}", 
    result.metrics.actionable_count, 
    result.metrics.total_count);
println!("Average complexity: {:.2}", 
    result.metrics.avg_complexity);
```

## 🧪 Testing & Quality

PDMT maintains high quality standards:

- **81%+ Test Coverage**: Comprehensive test suite
- **Fuzz Testing**: Automated robustness testing
- **Property Testing**: Invariant verification
- **Strict Linting**: clippy::pedantic + clippy::nursery

```bash
# Run tests with coverage
make test

# Run all quality checks
make all

# Run linting  
make lint

# Run formatting
make format

# Generate coverage report
make coverage
```

## 🤝 Contributing

We welcome contributions! Areas include:

- 🐛 **Bug Fixes**
- ✨ **New Features** 
- 📚 **Documentation**
- 🧪 **Testing**

### Development Setup

```bash
git clone https://github.com/paiml/pdmt
cd pdmt
cargo build --all-features
make test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- **Documentation**: [docs.rs/pdmt](https://docs.rs/pdmt)
- **Issues**: [GitHub Issues](https://github.com/paiml/pdmt/issues)