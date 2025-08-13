# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-01-13

### Added

- **Core templating engine** with Handlebars integration
- **Deterministic generation** with 0.0 temperature support
- **Todo validation system** with comprehensive quality checks
- **MCP (Model Context Protocol)** integration via PMCP SDK
- **Quality enforcement** via PMAT (Paiml MCP Agent Toolkit) integration
- **Multiple content formats**: YAML, JSON, Markdown, and plain text output
- **Dependency analysis**: Circular dependency detection and critical path calculation
- **Template inheritance** and composition support
- **Comprehensive validation**:
  - Actionability checks for todo items
  - Complexity scoring and thresholds  
  - Time estimation validation
  - SATD (Self-Admitted Technical Debt) detection
  - Generic language detection
- **Performance optimizations**:
  - Template compilation caching
  - Async processing support
  - Streaming content generation
- **Extensive testing suite**:
  - 81%+ test coverage achieved
  - Property-based testing with proptest
  - Fuzz testing with cargo-fuzz
  - Integration tests and edge case coverage
- **Development tooling**:
  - Makefile with lint, test, format, and fuzz targets
  - Strict linting with clippy::all, clippy::pedantic, clippy::nursery
  - Comprehensive error handling and reporting
- **Documentation**:
  - Complete API documentation
  - Usage examples and tutorials
  - Architecture overview and design decisions
- **Feature flags** for optional functionality:
  - `quality-proxy`: PMAT integration
  - `mcp-tools`: MCP protocol support  
  - `todo-validation`: Advanced todo validation
  - `property-tests`: Property testing utilities

### Features by Module

#### Template Engine (`src/template/`)
- Handlebars-based template compilation and rendering
- Custom helper functions for todo generation
- Template validation and error handling
- Built-in templates for common use cases
- Template metadata and versioning support

#### Content Models (`src/models/`)
- `Todo` and `TodoList` structures with rich metadata
- `GeneratedContent` with format conversion capabilities
- Quality reporting and validation results
- MCP request/response models
- Template definition models

#### Validation System (`src/validators/`)
- Todo-specific validation rules and quality metrics
- Structural validation for content integrity
- Dependency graph validation and cycle detection
- Custom validation configuration support
- Detailed issue reporting and suggestions

#### Quality Enforcement (`src/quality/`)
- PMAT proxy integration for quality gates
- Automatic refactoring and improvement suggestions
- Quality threshold enforcement
- Integration with external quality tools

#### MCP Integration (`src/mcp/`)
- Native Model Context Protocol support
- Tool execution and request handling
- PMCP SDK integration
- Async MCP communication

#### Error Handling (`src/error.rs`)
- Comprehensive error types for all operations
- Structured error reporting with context
- Error conversion and propagation
- User-friendly error messages

### Dependencies

- **Core**: `serde`, `serde_json`, `serde_yaml`, `handlebars`, `uuid`, `thiserror`
- **Async**: `tokio`, `futures`
- **Optional**: `chrono`, `regex`, `reqwest`, `proptest`, `pmcp-sdk`

### Performance

- Template compilation: ~1ms per template
- Todo validation (100 items): ~5ms  
- Content generation: ~2ms per operation
- Quality validation: ~10ms per validation

### Compatibility

- **Rust**: 1.70.0 or later
- **Operating Systems**: Linux, macOS, Windows
- **Architectures**: x86_64, aarch64

[Unreleased]: https://github.com/noahgift/pdmt/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/noahgift/pdmt/releases/tag/v1.0.0