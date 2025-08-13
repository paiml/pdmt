# Contributing to PDMT

Thank you for your interest in contributing to PDMT (Pragmatic Deterministic MCP Templating)! This document outlines the guidelines and processes for contributing to the project.

## 🌟 Ways to Contribute

- **🐛 Bug Reports**: Report issues you encounter
- **✨ Feature Requests**: Suggest new features or improvements
- **💻 Code Contributions**: Submit bug fixes or new features
- **📚 Documentation**: Improve documentation, examples, or guides
- **🧪 Testing**: Add test cases or improve test coverage
- **🎨 Examples**: Create usage examples or tutorials

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70.0 or later
- **Git**: For version control
- **Make**: For running development commands

### Setting Up Your Development Environment

1. **Fork the repository**
   ```bash
   # Click "Fork" on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/pdmt.git
   cd pdmt
   ```

2. **Install dependencies**
   ```bash
   cargo build --all-features
   ```

3. **Verify your setup**
   ```bash
   # Run tests
   make test
   
   # Run linting
   make lint
   
   # Run formatting
   make format
   ```

## 🔧 Development Workflow

### Making Changes

1. **Create a new branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number
   ```

2. **Make your changes**
   - Follow the coding standards outlined below
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   # Run all tests
   make test
   
   # Run specific tests
   cargo test test_name
   
   # Check linting
   make lint
   
   # Format code
   make format
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add new template validation feature"
   ```

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Test additions or modifications
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Build process or auxiliary tool changes

**Examples:**
```
feat(template): add custom helper support
fix(validation): resolve circular dependency detection
docs(readme): update installation instructions
test(todo): add edge case tests for validation
```

### Pull Request Process

1. **Ensure your branch is up to date**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**
   ```bash
   git push origin your-branch-name
   ```

3. **Create a Pull Request**
   - Go to GitHub and create a PR from your branch
   - Fill out the PR template completely
   - Link any related issues
   - Add appropriate labels

4. **Address review feedback**
   - Make requested changes
   - Push updates to your branch
   - Respond to reviewer comments

## 📋 Coding Standards

### General Guidelines

- **Follow Rust conventions**: Use `cargo fmt` and address `cargo clippy` warnings
- **Write clear, self-documenting code**: Use descriptive variable and function names
- **Add documentation**: Document all public APIs with examples
- **Include tests**: All new features should have corresponding tests
- **Maintain performance**: Consider performance implications of your changes

### Code Style

We use the standard Rust formatting with these additional guidelines:

```rust
// ✅ Good: Descriptive function names
pub fn validate_todo_actionability(todo: &Todo) -> ValidationResult {
    // Implementation
}

// ❌ Bad: Unclear abbreviations
pub fn val_t_act(t: &Todo) -> VResult {
    // Implementation
}

// ✅ Good: Comprehensive documentation
/// Validates a todo list for actionability and completeness.
///
/// This function checks each todo item for:
/// - Clear, actionable language
/// - Reasonable time estimates
/// - Valid dependencies
///
/// # Arguments
/// * `todo_list` - The todo list to validate
///
/// # Returns
/// A `ValidationResult` containing validation status and any issues found
///
/// # Examples
/// ```
/// use pdmt::{TodoList, TodoValidator};
/// 
/// let mut list = TodoList::new();
/// let validator = TodoValidator::new();
/// let result = validator.validate_todo_list(&list);
/// assert!(result.is_valid);
/// ```
pub fn validate_todo_list(todo_list: &TodoList) -> ValidationResult {
    // Implementation
}
```

### Testing Guidelines

- **Unit tests**: Test individual functions and methods
- **Integration tests**: Test feature interactions
- **Property tests**: Test invariants and edge cases
- **Documentation tests**: Ensure examples in docs work

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_todo_validation_basic() {
        let todo = Todo::new("Implement user authentication");
        let result = validate_todo(&todo);
        assert!(result.is_valid);
    }
    
    #[test]
    fn test_todo_validation_empty_content() {
        let todo = Todo::new("");
        let result = validate_todo(&todo);
        assert!(!result.is_valid);
        assert_eq!(result.issues[0].category, IssueCategory::Completeness);
    }
}
```

## 🧪 Testing Requirements

### Test Coverage

- Maintain **80%+ test coverage**
- Add tests for new features and bug fixes
- Test both happy path and error conditions
- Include edge cases and boundary conditions

### Running Tests

```bash
# All tests with coverage
make test

# Specific test
cargo test test_name

# Integration tests only
cargo test --test integration

# Property tests (if feature enabled)
cargo test --features property-tests

# Fuzz tests
make fuzz
```

## 📚 Documentation Standards

### API Documentation

- **Document all public items**: Functions, structs, enums, etc.
- **Include examples**: Show how to use the API
- **Explain parameters and return values**: Be clear about inputs and outputs
- **Note panics and errors**: Document when functions might panic or return errors

### README and Guides

- **Keep examples up to date**: Ensure code examples actually work
- **Use clear language**: Write for developers of all skill levels
- **Include complete examples**: Show imports, setup, and usage

## 🚨 Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **Rust version**: `rustc --version`
2. **PDMT version**: Version you're using
3. **Operating System**: OS and version
4. **Reproduction steps**: Minimal example that reproduces the issue
5. **Expected behavior**: What you expected to happen
6. **Actual behavior**: What actually happened
7. **Stack trace**: If there's a panic or error

**Template:**
```markdown
## Bug Description
Brief description of the bug.

## Environment
- Rust version: 1.75.0
- PDMT version: 1.0.0
- OS: Ubuntu 22.04

## Reproduction Steps
1. Create a new project with `cargo new test-project`
2. Add dependency: `pdmt = "1.0.0"`
3. Run the following code:
   ```rust
   // Minimal reproduction code
   ```

## Expected Behavior
The code should...

## Actual Behavior
Instead, it...

## Stack Trace
```
Error or panic message
```
```

### Feature Requests

When requesting features, please include:

1. **Use case**: Why do you need this feature?
2. **Proposed solution**: How do you envision it working?
3. **Alternatives considered**: What other approaches did you consider?
4. **Breaking changes**: Would this require breaking changes?

## 🏷️ Labels and Milestones

We use the following labels to categorize issues and PRs:

### Type Labels
- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to documentation
- `question`: Further information is requested

### Priority Labels  
- `priority: critical`: Needs immediate attention
- `priority: high`: Important but not critical
- `priority: medium`: Standard priority
- `priority: low`: Nice to have

### Status Labels
- `status: needs review`: Waiting for code review
- `status: needs work`: Changes requested
- `status: blocked`: Cannot proceed due to dependency

## 🎯 Areas for Contribution

We're particularly interested in contributions in these areas:

### High Priority
- **Performance optimizations**: Template compilation, validation speed
- **Documentation improvements**: Examples, guides, API docs
- **Test coverage**: Edge cases, integration tests, property tests
- **Bug fixes**: Issues marked with `bug` label

### Medium Priority  
- **New validators**: Additional validation rules for todos
- **Template features**: New Handlebars helpers, template inheritance
- **Error handling**: Better error messages and recovery
- **CLI tools**: Command-line utilities for PDMT

### Future Features
- **Plugin system**: Extensible validation and generation
- **Web interface**: GUI for template management
- **Performance monitoring**: Metrics and benchmarking tools

## 🤝 Community Guidelines

### Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome newcomers and diverse perspectives  
- **Be constructive**: Provide helpful feedback and suggestions
- **Be professional**: Keep discussions focused and on-topic

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and community discussion
- **Pull Requests**: Code contributions and reviews

### Getting Help

If you need help:

1. **Check the documentation**: [docs.rs/pdmt](https://docs.rs/pdmt)
2. **Search existing issues**: Your question might already be answered
3. **Create a new issue**: Use the question template
4. **Join discussions**: Participate in GitHub Discussions

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major version** (1.0.0): Breaking changes
- **Minor version** (1.1.0): New features, backward compatible
- **Patch version** (1.0.1): Bug fixes, backward compatible

### Release Criteria

Before releasing:

- ✅ All tests pass
- ✅ Documentation is updated
- ✅ CHANGELOG.md is updated
- ✅ Version numbers are updated
- ✅ Examples work with the new version

Thank you for contributing to PDMT! Your contributions help make this project better for everyone. 🎉