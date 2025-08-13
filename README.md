# PDMT - Pragmatic Deterministic MCP Templating

[![Crates.io](https://img.shields.io/crates/v/pdmt.svg)](https://crates.io/crates/pdmt)
[![Documentation](https://docs.rs/pdmt/badge.svg)](https://docs.rs/pdmt)
[![Build Status](https://github.com/noahshinn/pdmt/workflows/CI/badge.svg)](https://github.com/noahshinn/pdmt/actions)
[![Coverage](https://codecov.io/gh/noahshinn/pdmt/branch/main/graph/badge.svg)](https://codecov.io/gh/noahshinn/pdmt)
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

## 📖 Examples

The `examples/` directory contains comprehensive examples:

- **`todo_generation.rs`** - Basic todo list generation

Run examples with:

```bash
cargo run --example todo_generation --features="all"
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

# Run linting  
make lint

# Run formatting
make format
```

## 🤝 Contributing

We welcome contributions! Areas include:

- 🐛 **Bug Fixes**
- ✨ **New Features** 
- 📚 **Documentation**
- 🧪 **Testing**

### Development Setup

```bash
git clone https://github.com/noahshinn/pdmt
cd pdmt
cargo build --all-features
make test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- **Documentation**: [docs.rs/pdmt](https://docs.rs/pdmt)
- **Issues**: [GitHub Issues](https://github.com/noahshinn/pdmt/issues)