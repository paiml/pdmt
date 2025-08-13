# Deterministic MCP Templating Library Specification

**Version**: v1.0.0  
**Status**: SPECIFICATION  
**Created**: 2025-01-13  
**Target**: crates.io library extraction from assetgen  

## Executive Summary

The Deterministic MCP Templating (PDMT) library extracts the proven YAML templating system from AssetGen to create a reusable, high-quality Rust crate. This library provides deterministic content generation with integrated MCP tool support and PMAT quality enforcement, specifically designed for validating todo lists and other structured content with granular quality gates at each step.

## Problem Statement

Current limitations with existing templating approaches:

1. **Template Drift**: Templates and generated content can diverge over time
2. **Quality Variance**: No enforced quality standards for AI-generated content  
3. **Non-Deterministic Results**: AI outputs vary between runs without structural validation
4. **Fragmented Tooling**: Template validation, MCP integration, and quality gates exist separately
5. **Limited Reusability**: AssetGen's templating system is tightly coupled to course generation

## Solution Architecture

### Core Concept

PDMT provides a unified library for deterministic templating with quality enforcement:

```
Template Definition → Content Generation → Quality Validation → MCP Integration → Final Output
```

### Key Components

1. **Template Engine** (`src/template/`)
   - YAML-based template definitions with schema validation
   - Deterministic content generation with 0.0 temperature
   - Template inheritance and composition
   - Built-in validation rules and constraints

2. **Quality Integration** (`src/quality/`)
   - PMAT quality-gates proxy mode integration
   - Configurable complexity thresholds
   - SATD (Self-Admitted Technical Debt) detection
   - Automated refactoring suggestions

3. **MCP Tools** (`src/mcp/`)
   - Native MCP tool definitions for template operations
   - PMCP SDK integration for seamless Claude Code usage
   - Streaming support for large templates
   - Progress reporting and cancellation

4. **Content Validators** (`src/validators/`)
   - Todo list structure validation
   - Granular task detail verification
   - Progress tracking validation
   - Cross-reference integrity checks

## Detailed Design

### Library Architecture

```rust
// Core library structure
pub mod template {
    pub mod engine;      // Template parsing and rendering
    pub mod schema;      // Template schema validation
    pub mod inheritance; // Template extension system
}

pub mod quality {
    pub mod gates;       // Quality gate integration
    pub mod proxy;       // PMAT proxy mode
    pub mod metrics;     // Quality metrics collection
}

pub mod mcp {
    pub mod tools;       // MCP tool definitions
    pub mod handlers;    // Request handlers
    pub mod streaming;   // Streaming support
}

pub mod validators {
    pub mod todo;        // Todo list validation
    pub mod structure;   // Content structure validation
    pub mod references;  // Cross-reference validation
}

pub mod models {
    pub mod template;    // Template data structures
    pub mod content;     // Generated content models
    pub mod quality;     // Quality report models
}
```

### Template Definition Format

Templates follow the proven AssetGen YAML structure:

```yaml
# todo-list-template.yml
id: todo_list
version: "1.0.0"
extends: base

metadata:
  provider: "deterministic"
  description: "Deterministic todo list generation with quality enforcement"
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
      enum: ["high", "medium", "low"]
      default: "high"
      description: "Level of task detail"

output_schema:
  format: yaml
  structure: |
    todos:
      - id: string (uuid v4)
        content: string (1-100 chars, specific and actionable)
        status: enum [pending, in_progress, completed]
        priority: enum [low, medium, high, critical]
        estimated_hours: number (0.5-40)
        dependencies: [string] (other todo ids)
        quality_gates:
          complexity_check: boolean
          completeness_check: boolean
          actionability_check: boolean

validation:
  deterministic_only: true
  required_fields: ["todos"]
  quality_gates:
    max_complexity_per_task: 8
    require_time_estimates: true
    require_specific_actions: true
    min_task_detail_chars: 10
    max_task_detail_chars: 100
  structure_rules:
    max_todos_per_batch: 50
    require_dependency_graph: true
    prevent_circular_dependencies: true

prompt_template: |
  Generate a deterministic todo list for project "{{project_name}}" with {{granularity}} granularity.
  
  Requirements to address:
  {{#each requirements}}
  - {{this}}
  {{/each}}
  
  Rules:
  1. Each todo must be specific and actionable
  2. Provide realistic time estimates
  3. Include clear dependencies
  4. Use consistent ID format
  5. No generic or vague tasks
  
  Output as valid YAML matching the schema.

quality_enforcement:
  pmat_config:
    mode: "strict"
    max_complexity: 8
    allow_satd: false
    require_docs: true
    auto_format: true
```

### Core Template Engine

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDefinition {
    pub id: String,
    pub version: String,
    pub extends: Option<String>,
    pub metadata: TemplateMetadata,
    pub input_schema: serde_json::Value,
    pub output_schema: OutputSchema,
    pub validation: ValidationRules,
    pub prompt_template: String,
    pub quality_enforcement: QualityEnforcement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub provider: String, // "deterministic", "anthropic", etc.
    pub description: String,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

pub struct TemplateEngine {
    templates: std::collections::HashMap<String, TemplateDefinition>,
    handlebars: handlebars::Handlebars<'static>,
    quality_proxy: Option<crate::quality::QualityProxy>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            templates: std::collections::HashMap::new(),
            handlebars: handlebars::Handlebars::new(),
            quality_proxy: None,
        }
    }

    /// Load template from YAML string or file
    pub fn load_template(&mut self, yaml: &str) -> Result<(), TemplateError> {
        let template: TemplateDefinition = serde_yaml::from_str(yaml)?;
        self.register_template(template)
    }

    /// Generate deterministic content from template
    pub async fn generate<T>(&self, template_id: &str, input: T) -> Result<GeneratedContent, TemplateError> 
    where
        T: Serialize,
    {
        let template = self.get_template(template_id)?;
        
        // 1. Validate input against schema
        self.validate_input(&template, &input)?;
        
        // 2. Render template (deterministic)
        let rendered = self.render_template(&template, &input)?;
        
        // 3. Apply quality gates if configured
        let content = if let Some(proxy) = &self.quality_proxy {
            proxy.validate_and_refactor(&rendered, &template.quality_enforcement).await?
        } else {
            rendered
        };
        
        // 4. Validate output structure
        self.validate_output(&template, &content)?;
        
        Ok(GeneratedContent {
            id: Uuid::new_v4(),
            template_id: template_id.to_string(),
            content,
            quality_report: QualityReport::default(),
            generated_at: chrono::Utc::now(),
        })
    }
}
```

### MCP Integration

```rust
use pmcp::{Client, Server, tools::Tool, types::*};

/// MCP tool for deterministic template generation
pub fn create_template_tool() -> Tool {
    Tool::new("deterministic_template")
        .with_description("Generate deterministic content using PDMT templates")
        .with_input_schema(json!({
            "type": "object",
            "required": ["template_id", "input"],
            "properties": {
                "template_id": {
                    "type": "string",
                    "description": "ID of the template to use"
                },
                "input": {
                    "type": "object", 
                    "description": "Input data for template generation"
                },
                "quality_mode": {
                    "type": "string",
                    "enum": ["strict", "advisory", "auto_fix"],
                    "default": "strict",
                    "description": "Quality enforcement mode"
                },
                "output_format": {
                    "type": "string", 
                    "enum": ["yaml", "json", "markdown"],
                    "default": "yaml",
                    "description": "Output format"
                }
            }
        }))
}

/// Handle template generation MCP requests
pub async fn handle_template_request(
    engine: &TemplateEngine,
    request: ToolRequest,
) -> Result<ToolResponse, McpError> {
    let args = request.arguments;
    let template_id = args.get("template_id").and_then(|v| v.as_str())
        .ok_or(McpError::InvalidInput("template_id required".into()))?;
    
    let input = args.get("input")
        .ok_or(McpError::InvalidInput("input required".into()))?;
    
    let quality_mode = args.get("quality_mode")
        .and_then(|v| v.as_str())
        .unwrap_or("strict");
        
    // Generate content with quality enforcement
    let result = engine.generate(template_id, input).await
        .map_err(|e| McpError::ToolError(e.to_string()))?;
    
    Ok(ToolResponse {
        content: vec![TextContent {
            type_: "text".to_string(),
            text: serde_yaml::to_string(&result.content)?,
        }],
        is_error: false,
    })
}
```

### Todo List Validation Example

```rust
/// Specialized validator for todo lists
pub struct TodoValidator {
    max_todos: usize,
    require_estimates: bool,
    complexity_threshold: u32,
}

impl TodoValidator {
    pub fn validate_todo_list(&self, todos: &[Todo]) -> ValidationResult {
        let mut issues = Vec::new();
        
        // Check todo count
        if todos.len() > self.max_todos {
            issues.push(ValidationIssue::error(
                "too_many_todos",
                format!("Todo count {} exceeds maximum {}", todos.len(), self.max_todos)
            ));
        }
        
        // Validate each todo
        for (idx, todo) in todos.iter().enumerate() {
            // Check content specificity
            if todo.content.len() < 10 {
                issues.push(ValidationIssue::warning(
                    "vague_todo",
                    format!("Todo {} is too vague: '{}'", idx, todo.content)
                ));
            }
            
            // Check for actionable language
            if !self.is_actionable(&todo.content) {
                issues.push(ValidationIssue::error(
                    "not_actionable", 
                    format!("Todo {} is not actionable: '{}'", idx, todo.content)
                ));
            }
            
            // Validate time estimates
            if self.require_estimates && todo.estimated_hours.is_none() {
                issues.push(ValidationIssue::error(
                    "missing_estimate",
                    format!("Todo {} missing time estimate", idx)
                ));
            }
        }
        
        // Check dependency graph
        if let Err(cycle) = self.check_dependency_cycles(todos) {
            issues.push(ValidationIssue::error(
                "circular_dependency",
                format!("Circular dependency detected: {}", cycle.join(" -> "))
            ));
        }
        
        ValidationResult {
            is_valid: issues.iter().all(|i| i.severity != Severity::Error),
            issues,
            metrics: self.calculate_metrics(todos),
        }
    }
    
    fn is_actionable(&self, content: &str) -> bool {
        let actionable_verbs = ["implement", "create", "build", "write", "add", 
                               "remove", "update", "fix", "test", "deploy"];
        let lower_content = content.to_lowercase();
        actionable_verbs.iter().any(|verb| lower_content.starts_with(verb))
    }
}
```

### Quality Integration with PMAT

```rust
use crate::quality::proxy::QualityProxy;

/// Integration with PMAT quality gates
pub struct PmatQualityProxy {
    client: QualityProxyClient,
    config: QualityConfig,
}

impl PmatQualityProxy {
    pub async fn validate_and_refactor(
        &self, 
        content: &str, 
        enforcement: &QualityEnforcement
    ) -> Result<String, QualityError> {
        let request = ProxyRequest {
            operation: "validate".to_string(),
            content: content.to_string(),
            mode: enforcement.pmat_config.mode.clone(),
            quality_config: enforcement.pmat_config.clone(),
        };
        
        let response = self.client.process(request).await?;
        
        match response.status.as_str() {
            "accepted" => Ok(content.to_string()),
            "modified" => Ok(response.final_content),
            "rejected" => Err(QualityError::QualityGateFailed {
                violations: response.quality_report.violations,
                suggestions: response.quality_report.suggestions,
            }),
            _ => Err(QualityError::UnknownResponse(response.status)),
        }
    }
}
```

## Usage Examples

### Example 1: Basic Todo List Generation

```rust
use pdmt::{TemplateEngine, TodoValidator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize template engine
    let mut engine = TemplateEngine::new();
    engine.load_template_file("templates/todo-list.yml").await?;
    
    // Enable PMAT quality proxy
    engine.enable_quality_proxy(QualityConfig {
        mode: QualityMode::Strict,
        max_complexity: 8,
        require_estimates: true,
    })?;
    
    // Generate deterministic todo list
    let input = serde_json::json!({
        "project_name": "Rust Web API",
        "requirements": [
            "Create user authentication system",
            "Implement REST API endpoints",
            "Add database integration",
            "Write comprehensive tests"
        ],
        "granularity": "high"
    });
    
    let result = engine.generate("todo_list", input).await?;
    println!("{}", serde_yaml::to_string(&result.content)?);
    
    Ok(())
}
```

### Example 2: MCP Tool Integration 

```rust
// Claude Code usage example
use pdmt::mcp::create_template_tool;

// Register PDMT tool with MCP server
let template_tool = create_template_tool();
mcp_server.register_tool(template_tool);

// Claude Code can now use the tool:
// Call: deterministic_template
// Args: {
//   "template_id": "todo_list", 
//   "input": {"project_name": "My Project", "requirements": ["..."]}
// }
```

### Example 3: Custom Template Creation

```yaml
# custom-project-template.yml
id: project_scaffold
version: "1.0.0"
extends: base

metadata:
  provider: "deterministic"
  description: "Generate project scaffold with quality gates"
  parameters:
    temperature: 0.0

input_schema:
  type: object
  required: ["project_type", "features"]
  properties:
    project_type:
      type: string
      enum: ["web", "cli", "library", "service"]
    features:
      type: array
      items: {type: string}

validation:
  deterministic_only: true
  quality_gates:
    require_readme: true
    require_tests: true
    max_file_count: 100

prompt_template: |
  Generate a {{project_type}} project structure with these features:
  {{#each features}}
  - {{this}}
  {{/each}}
  
  Include:
  1. Cargo.toml with appropriate dependencies
  2. src/ structure with main entry points
  3. tests/ directory with example tests
  4. README.md with usage instructions
  5. .gitignore with Rust-specific entries
```

## Testing Strategy

### Property Tests (using proptest)

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn template_generation_is_deterministic(
            template_id in "todo_list|project_scaffold",
            input in any::<ProjectInput>()
        ) {
            let mut engine1 = TemplateEngine::new();
            let mut engine2 = TemplateEngine::new();
            
            let result1 = engine1.generate(&template_id, &input).await?;
            let result2 = engine2.generate(&template_id, &input).await?;
            
            // Results should be identical for deterministic templates
            prop_assert_eq!(result1.content, result2.content);
        }
        
        #[test] 
        fn generated_todos_meet_quality_standards(
            requirements in prop::collection::vec("[a-zA-Z ]+", 1..10)
        ) {
            let input = json!({
                "project_name": "Test Project",
                "requirements": requirements,
                "granularity": "high"
            });
            
            let result = engine.generate("todo_list", input).await?;
            let todos: TodoList = serde_yaml::from_str(&result.content)?;
            
            for todo in &todos.todos {
                // Each todo should be actionable
                prop_assert!(todo.content.len() >= 10);
                prop_assert!(todo.content.len() <= 100);
                
                // Should have time estimate
                prop_assert!(todo.estimated_hours.is_some());
                
                // Should be specific (no generic words)
                let generic_words = ["thing", "stuff", "item", "something"];
                let lower_content = todo.content.to_lowercase();
                prop_assert!(!generic_words.iter().any(|word| lower_content.contains(word)));
            }
        }
    }
}
```

### Example Programs

1. **`examples/todo_generation.rs`**
   - Interactive todo list generation with CLI
   - Demonstrates quality gate integration
   - Shows different granularity levels

2. **`examples/mcp_integration.rs`**
   - Full MCP server with PDMT tools
   - Claude Code integration example
   - Streaming and progress reporting

3. **`examples/custom_template.rs`**
   - Creating and using custom templates
   - Template inheritance patterns
   - Advanced validation rules

4. **`examples/quality_proxy_demo.rs`**
   - PMAT quality proxy integration
   - Different enforcement modes
   - Quality metrics collection

## Performance Considerations

### Caching Strategy
- Template compilation results cached in memory
- AST parsing results cached with LRU eviction
- Quality gate results cached by content hash
- MCP tool definitions cached at startup

### Streaming Support  
- Large template generation with progress callbacks
- Chunked output for memory efficiency
- Async processing with backpressure handling
- MCP streaming protocol compliance

### Concurrency
- Thread-safe template engine with Arc<RwLock<>>
- Async quality gate validation
- Parallel processing of todo batches
- Rate limiting for external quality services

## Quality Assurance

### Code Coverage Target: 80%
- Unit tests for all public APIs
- Integration tests for MCP tools
- Property tests for deterministic behavior
- Doc tests for all examples

### Quality Gates (Self-Applied)
- Maximum function complexity: 10
- Zero SATD comments allowed
- All public functions documented
- Clippy warnings = errors
- Automated rustfmt on save

### Performance Benchmarks
- Template generation: <10ms for 50 todos
- Quality validation: <100ms per operation  
- MCP tool response: <200ms total
- Memory usage: <50MB for typical workload

## Crates.io Publication

### Crate Metadata
```toml
[package]
name = "pdmt"
version = "1.0.0"
edition = "2021"
authors = ["Pragmatic AI Labs <team@paiml.com>"]
description = "Deterministic MCP templating library with quality enforcement"
documentation = "https://docs.rs/pdmt"
repository = "https://github.com/paiml/pdmt"
license = "MIT OR Apache-2.0"
keywords = ["template", "mcp", "quality", "deterministic", "todo"]
categories = ["template-engine", "development-tools", "text-processing"]
readme = "README.md"

[features]
default = ["quality-proxy", "mcp-tools"]
quality-proxy = ["pmat-integration"]
mcp-tools = ["pmcp"]
todo-validation = ["uuid", "chrono"]
```

### Dependencies
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
handlebars = { version = "6.0", features = ["script_helper"] }
uuid = { version = "1.0", features = ["v4"], optional = true }
chrono = { version = "0.4", features = ["serde"], optional = true }
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"

# MCP integration
pmcp = { version = "1.0", optional = true }

# Quality integration  
pmat = { version = "2.0", optional = true }

# Async support
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"

[dev-dependencies]
proptest = "1.0"
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3.0"
pretty_assertions = "1.4"
```

## Migration Path

### Phase 1: Core Library (Weeks 1-2)
1. Extract template engine from assetgen
2. Implement basic YAML template support  
3. Add deterministic generation
4. Create initial todo list validator
5. Basic unit tests (50% coverage)

### Phase 2: Quality Integration (Week 3)
1. Integrate PMAT quality proxy
2. Add quality gate enforcement
3. Implement auto-refactoring hooks
4. Property tests for quality invariants
5. Increase test coverage to 70%

### Phase 3: MCP Integration (Week 4)
1. Add PMCP SDK integration
2. Create MCP tool definitions
3. Implement streaming support
4. Add progress reporting
5. Complete integration tests

### Phase 4: Production Ready (Week 5)
1. Performance optimization and benchmarks
2. Complete documentation with examples
3. Increase test coverage to 80%+
4. Security audit and validation
5. Prepare crates.io publication

## Success Metrics

### Quality Metrics
- 100% deterministic generation (same input = same output)
- Zero quality gate failures in strict mode
- All generated todos meet actionability standards
- Template compilation success rate: 100%

### Performance Metrics  
- Template generation latency: <10ms (p95)
- Quality validation latency: <100ms (p95)
- MCP tool response time: <200ms (p95)
- Memory usage: <50MB per engine instance

### Adoption Metrics
- Published to crates.io within 5 weeks
- Documentation completeness: 100% public APIs
- Test coverage: >80% line coverage  
- Example programs: 4+ comprehensive examples

## Conclusion

The PDMT library represents a significant advancement in deterministic content generation with integrated quality enforcement. By extracting and enhancing AssetGen's proven templating system, we create a reusable, high-quality Rust crate that enables AI agents to generate consistently high-quality structured content while maintaining deterministic behavior and enforcing quality gates at every step.

The integration with MCP tools and PMAT quality proxy ensures that this library can be seamlessly adopted by Claude Code and other AI development tools, providing a robust foundation for AI-assisted content generation with built-in quality assurance.