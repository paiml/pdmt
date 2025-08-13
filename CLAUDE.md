# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PDMT (Pragmatic Deterministic MCP Templating) is a high-performance Rust library for deterministic templating with Model Context Protocol (MCP) integration. The library specializes in generating validated todo lists and structured content with quality enforcement through PMAT integration.

**Key Purpose**: Provide deterministic (0.0 temperature) content generation with comprehensive validation, quality gates, and MCP tool support for AI coding assistants.

## Development Commands

### Core Development Workflow
```bash
# Development setup
make install-tools              # Install required development tools
cargo build --all-features     # Build with all features

# Testing (requires 80% coverage minimum)
make test                      # Run tests with coverage report  
make test-unit                 # Unit tests only
make test-integration          # Integration tests only
make test-doc                  # Documentation tests

# Code Quality
make format                    # Format code with rustfmt
make lint                      # Run clippy with strict rules
make audit                     # Security audit

# Full quality check
make check-release             # Complete release validation (format, lint, test, audit)
```

### Single Test Execution
```bash
# Run specific test
cargo test test_name -- --nocapture

# Run tests in specific module
cargo test models::todo --lib

# Run with specific feature set
cargo test --features "todo-validation" --test integration
```

### Examples and Benchmarks
```bash
# Run main example
cargo run --example todo_generation --features="todo-validation"

# Run benchmarks
make bench
cargo bench --all-features
```

## Architecture Overview

PDMT uses a modular feature-based architecture where functionality is gated behind feature flags:

### Core Modules Structure
- **`src/template/`** - Template engine using Handlebars with deterministic generation
  - `engine.rs` - Main template processing engine
  - `definition.rs` - YAML template definitions and schemas  
  - `inheritance.rs` - Template composition and extension
  - `schema.rs` - JSON schema validation for templates

- **`src/models/`** - Data structures for all content types
  - `todo.rs` - Todo lists, validation rules, dependency graphs
  - `content.rs` - Generated content with metadata and format conversion
  - `template.rs` - Template definitions and metadata
  - `quality.rs` - Quality reports and validation results

- **`src/validators/`** - Content validation logic
  - `todo.rs` - Todo-specific validation (actionability, complexity, estimates)
  - `structure.rs` - Generic content structure validation

- **`src/quality/`** - PMAT integration (requires `quality-proxy` feature)
  - `proxy.rs` - HTTP client for quality-gates service

- **`src/mcp/`** - Model Context Protocol tools (requires `mcp-tools` feature)  
  - `tools.rs` - MCP tool definitions for template operations

### Feature Flags
- **Default features**: `["quality-proxy", "mcp-tools", "todo-validation"]`
- **`todo-validation`** - Advanced todo validation with UUID and timestamps
- **`quality-proxy`** - PMAT quality enforcement via HTTP proxy
- **`mcp-tools`** - MCP/PMCP integration for AI assistants
- **`property-tests`** - Property-based testing with proptest
- **`streaming`** - Async streaming support

### Template System
Templates are YAML-based with Handlebars rendering:
- Templates stored in `templates/` directory
- Built-in templates compiled into binary
- Support for template inheritance and composition
- JSON schema validation for inputs
- Deterministic generation enforced via 0.0 temperature parameter

### Quality Gates Integration
- Integration with PMAT (Paiml MCP Agent Toolkit) for quality enforcement
- Configurable complexity thresholds and validation rules
- SATD (Self-Admitted Technical Debt) detection
- Circular dependency detection in todo lists
- Actionability scoring for todo items

## Testing Strategy

The project maintains 81%+ test coverage with multiple testing approaches:

### Test Structure
- **Unit tests**: In `src/` modules using `#[cfg(test)]`
- **Integration tests**: In `tests/` directory for full workflow testing
- **Property tests**: Fuzzing and invariant checking (when `property-tests` feature enabled)
- **Fuzz tests**: In `fuzz/` directory using cargo-fuzz
- **Benchmark tests**: In `benches/` directory using criterion

### Quality Requirements
- **Minimum 80% test coverage** enforced via `make test`
- **Zero clippy warnings** with pedantic and nursery lints
- **Strict formatting** with rustfmt
- **Security audits** required for dependencies

## Development Notes

### MSRV Policy
- **Minimum Supported Rust Version**: 1.82.0
- Required for ICU dependencies and PMCP integration
- Enforced in CI/CD pipeline

### Error Handling
- Uses `thiserror` for structured error types in `src/error.rs`
- Custom error types for each module (TemplateError, ValidationError, etc.)
- Comprehensive error context preservation

### Async Architecture  
- Built on tokio runtime with async/await throughout
- Template generation is async for streaming and cancellation support
- MCP integration requires async for protocol compatibility

### Performance Considerations
- Template caching and reuse in engine
- Lazy loading of built-in templates
- Optimized release profile with LTO and minimal codegen units
- Benchmark suite for performance regression detection

### MCP Integration Pattern
When working with MCP tools, the library provides native tool definitions that can be registered with MCP servers:

```rust
use pdmt::mcp::create_template_tool;

// Register template tool with MCP server
let template_tool = create_template_tool();
mcp_server.register_tool(template_tool);
```

This enables AI assistants to use PDMT functionality through standardized MCP protocols.