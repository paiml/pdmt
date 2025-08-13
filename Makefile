# PDMT Makefile - Development tooling

.PHONY: help
help: ## Show this help message
	@echo "PDMT - Deterministic MCP Templating Library"
	@echo ""
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Main targets
.PHONY: all
all: format lint test ## Run format, lint, and test

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf .pdmt-cache/
	rm -rf proptest-regressions/

# Formatting
.PHONY: format
format: ## Format all Rust code using rustfmt
	@echo "🎨 Formatting code..."
	cargo fmt --all
	@echo "✅ Code formatted successfully"

.PHONY: format-check
format-check: ## Check if code is formatted correctly
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check
	@echo "✅ Code formatting is correct"

# Linting
.PHONY: lint
lint: ## Run all linting checks
	@echo "🔍 Running linting checks..."
	@cargo clippy --all-targets --all-features 2>&1 | grep -E "^error:" || true
	@if cargo clippy --all-targets --all-features 2>&1 | grep -q "^error:"; then \
		echo "❌ Linting errors found"; \
		exit 1; \
	else \
		echo "✅ All linting checks passed (warnings allowed)"; \
	fi

.PHONY: lint-fix
lint-fix: ## Attempt to automatically fix linting issues
	@echo "🔧 Auto-fixing linting issues..."
	cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
	@echo "✅ Auto-fix completed"

# Testing
.PHONY: test
test: ## Run all tests with coverage report
	@echo "🧪 Running tests with coverage..."
	@cargo tarpaulin --all-features --workspace --timeout 120 --skip-clean --fail-under 80 || \
		(echo "❌ Tests failed or coverage is below 80%"; exit 1)
	@echo "✅ All tests passed with ≥80% coverage"

.PHONY: test-unit
test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	cargo test --lib --all-features
	@echo "✅ Unit tests passed"

.PHONY: test-integration
test-integration: ## Run integration tests only
	@echo "🧪 Running integration tests..."
	cargo test --test '*' --all-features
	@echo "✅ Integration tests passed"

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "🧪 Running documentation tests..."
	cargo test --doc --all-features
	@echo "✅ Documentation tests passed"

.PHONY: test-examples
test-examples: ## Test that all examples compile and run
	@echo "🧪 Testing examples..."
	cargo run --example todo_generation --features todo-validation -- --project "Test Project" --requirements "Test req1,Test req2" --granularity high --max-todos 5 --output-format yaml
	@echo "✅ Examples tested successfully"

# Coverage
.PHONY: coverage
coverage: ## Generate detailed test coverage report
	@echo "📊 Generating detailed coverage report..."
	@cargo tarpaulin --all-features --workspace --timeout 120 --skip-clean \
		--out Html --output-dir target/coverage --fail-under 80
	@echo "✅ Coverage report generated in target/coverage/"
	@echo "📊 View report: open target/coverage/tarpaulin-report.html"

# Benchmarks
.PHONY: bench
bench: ## Run benchmarks
	@echo "⚡ Running benchmarks..."
	cargo bench --all-features
	@echo "✅ Benchmarks completed"

# Documentation
.PHONY: docs
docs: ## Generate documentation
	@echo "📚 Generating documentation..."
	cargo doc --all-features --no-deps --document-private-items
	@echo "✅ Documentation generated"

.PHONY: docs-open
docs-open: docs ## Generate and open documentation
	@echo "📖 Opening documentation..."
	cargo doc --all-features --no-deps --open

# Security
.PHONY: audit
audit: ## Run security audit
	@echo "🔒 Running security audit..."
	cargo audit
	@echo "✅ Security audit passed"

# Release preparation
.PHONY: check-release
check-release: format lint test audit ## Full release check
	@echo "🚀 Running full release check..."
	cargo check --release --all-features
	cargo build --release --all-features
	@echo "✅ Release check passed"

# Development
.PHONY: dev
dev: ## Run in development mode with file watching
	@echo "🔥 Starting development mode..."
	cargo watch -x 'run --example todo_generation --features todo-validation'

.PHONY: install-tools
install-tools: ## Install development tools
	@echo "🔧 Installing development tools..."
	rustup component add rustfmt clippy
	cargo install cargo-watch cargo-tarpaulin cargo-audit --locked || true
	@echo "✅ Development tools installed"

# Build variants
.PHONY: build
build: ## Build the project
	@echo "🔨 Building project..."
	cargo build --all-features
	@echo "✅ Build completed"

.PHONY: build-release
build-release: ## Build optimized release version
	@echo "🔨 Building release version..."
	cargo build --release --all-features
	@echo "✅ Release build completed"

# Feature-specific builds
.PHONY: build-minimal
build-minimal: ## Build with minimal features
	@echo "🔨 Building minimal version..."
	cargo build --no-default-features
	@echo "✅ Minimal build completed"

.PHONY: build-full
build-full: ## Build with all features
	@echo "🔨 Building with all features..."
	cargo build --all-features
	@echo "✅ Full build completed"

# CI/CD helpers
.PHONY: ci-check
ci-check: format-check lint test ## CI check pipeline
	@echo "🤖 Running CI checks..."
	@echo "✅ All CI checks passed"

.PHONY: pre-commit
pre-commit: format lint test-unit ## Pre-commit checks
	@echo "✨ Running pre-commit checks..."
	@echo "✅ Pre-commit checks passed"