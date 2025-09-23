# unvenv - Development Workflow
# Requires: just, peter-hook, versioneer

# Default recipe to display available commands
default:
    @just --list

# Setup development environment
setup:
    @echo "Setting up unvenv development environment..."
    @just install-hooks
    @echo "✅ Setup complete!"

# Install git hooks using peter-hook
install-hooks:
    @echo "Installing git hooks with peter-hook..."
    @if command -v peter-hook >/dev/null 2>&1; then \
        peter-hook install; \
        echo "✅ Git hooks installed"; \
    else \
        echo "❌ peter-hook not found. Install with: cargo install peter-hook"; \
        exit 1; \
    fi

# Version management
version-show:
    @echo "Current version: $(cat VERSION)"
    @echo "Cargo.toml version: $(grep '^version' Cargo.toml | cut -d'"' -f2)"

# Bump patch version (0.1.0 -> 0.1.1)
version-patch:
    @echo "Bumping patch version..."
    @if command -v versioneer >/dev/null 2>&1; then \
        versioneer patch; \
        echo "✅ Version bumped to: $(cat VERSION)"; \
    else \
        echo "❌ versioneer not found. Install with: cargo install versioneer"; \
        exit 1; \
    fi

# Bump minor version (0.1.0 -> 0.2.0)
version-minor:
    @echo "Bumping minor version..."
    @if command -v versioneer >/dev/null 2>&1; then \
        versioneer minor; \
        echo "✅ Version bumped to: $(cat VERSION)"; \
    else \
        echo "❌ versioneer not found. Install with: cargo install versioneer"; \
        exit 1; \
    fi

# Bump major version (0.1.0 -> 1.0.0)
version-major:
    @echo "Bumping major version..."
    @if command -v versioneer >/dev/null 2>&1; then \
        versioneer major; \
        echo "✅ Version bumped to: $(cat VERSION)"; \
    else \
        echo "❌ versioneer not found. Install with: cargo install versioneer"; \
        exit 1; \
    fi

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    @rm -rf target/
    @rm -f Cargo.lock
    @echo "✅ Clean complete!"

# Deep clean (including dependencies cache)
clean-all: clean
    @echo "Deep cleaning (including cargo cache)..."
    @rm -rf ~/.cargo/registry/cache/
    @echo "✅ Deep clean complete!"

# Build in debug mode
build:
    @echo "Building unvenv..."
    cargo build
    @echo "✅ Build complete!"

# Build in release mode
build-release:
    @echo "Building unvenv in release mode..."
    cargo build --release
    @echo "✅ Release build complete!"

# Build for all targets (cross-compilation)
build-all-targets:
    @echo "Building for all targets..."
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo build --release --target aarch64-unknown-linux-gnu
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    cargo build --release --target x86_64-pc-windows-msvc
    @echo "✅ All targets built!"

# Run tests
test:
    @echo "Running tests..."
    cargo test --all --verbose
    @echo "✅ Tests complete!"

# Run tests with coverage
test-coverage:
    @echo "Running tests with coverage..."
    @if command -v cargo-tarpaulin >/dev/null 2>&1; then \
        cargo tarpaulin --all --out xml --engine llvm --timeout 300; \
    else \
        echo "❌ cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"; \
        exit 1; \
    fi

# Code quality checks
quality: format-check lint audit

# Format code (requires nightly rustfmt)
format:
    @echo "Formatting code..."
    @if rustup toolchain list | grep -q nightly; then \
        cargo +nightly fmt; \
        echo "✅ Code formatted"; \
    else \
        echo "❌ Nightly toolchain required for formatting"; \
        echo "Install with: rustup install nightly"; \
        exit 1; \
    fi

# Check code formatting
format-check:
    @echo "Checking code formatting..."
    @if rustup toolchain list | grep -q nightly; then \
        cargo +nightly fmt --all -- --check; \
        echo "✅ Formatting check passed"; \
    else \
        echo "❌ Nightly toolchain required for formatting"; \
        echo "Install with: rustup install nightly"; \
        exit 1; \
    fi

# Lint code with clippy
lint:
    @echo "Linting code..."
    cargo clippy --all-targets -- -D warnings
    @echo "✅ Linting complete!"

# Security audit
audit:
    @echo "Running security audit..."
    @if command -v cargo-audit >/dev/null 2>&1; then \
        cargo audit; \
        echo "✅ Security audit passed"; \
    else \
        echo "❌ cargo-audit not found. Install with: cargo install cargo-audit"; \
        exit 1; \
    fi

# Dependency compliance check
deny:
    @echo "Checking dependency compliance..."
    @if command -v cargo-deny >/dev/null 2>&1; then \
        cargo deny check; \
        echo "✅ Dependency compliance check passed"; \
    else \
        echo "❌ cargo-deny not found. Install with: cargo install cargo-deny"; \
        exit 1; \
    fi

# Full CI pipeline (what runs in GitHub Actions)
ci: quality test build-release
    @echo "✅ Full CI pipeline complete!"

# Development workflow - quick checks before commit
dev: format lint test
    @echo "✅ Development checks complete! Ready to commit."

# Release workflow
release: clean quality test build-release
    @echo "✅ Release workflow complete!"

# Install development dependencies
install-deps:
    @echo "Installing development dependencies..."
    @echo "Installing Rust nightly (for rustfmt)..."
    rustup install nightly
    @echo "Installing peter-hook..."
    cargo install peter-hook
    @echo "Installing versioneer..."
    cargo install versioneer
    @echo "Installing cargo tools..."
    cargo install cargo-audit
    cargo install cargo-deny
    cargo install cargo-tarpaulin
    @echo "✅ All development dependencies installed!"

# Show project info
info:
    @echo "unvenv (unvenv)"
    @echo "Description: A CLI tool written in Rust"
    @echo "Author: Your Name <your.email@example.com>"
    @echo "Repository: https://github.com/yourusername/unvenv"
    @echo "License: MIT"
    @echo "Version: $(cat VERSION)"
    @echo "Rust Edition: 2021"
    @echo "MSRV: 1.70.0"

# Run the built binary
run *args:
    cargo run -- {{ args }}

# Run the binary with release optimizations
run-release *args:
    cargo run --release -- {{ args }}

# Profile the application (requires cargo-flamegraph)
profile *args:
    @if command -v cargo-flamegraph >/dev/null 2>&1; then \
        cargo flamegraph -- {{ args }}; \
    else \
        echo "❌ cargo-flamegraph not found. Install with: cargo install flamegraph"; \
        exit 1; \
    fi

# Benchmark the application (if benchmarks exist)
bench:
    @if [ -d "benches" ]; then \
        cargo bench; \
    else \
        echo "No benchmarks found in benches/ directory"; \
    fi

# Generate and open documentation
docs:
    @echo "Generating documentation..."
    cargo doc --open --no-deps
    @echo "✅ Documentation generated and opened!"

# Check for unused dependencies
unused-deps:
    @if command -v cargo-machete >/dev/null 2>&1; then \
        cargo machete; \
    else \
        echo "❌ cargo-machete not found. Install with: cargo install cargo-machete"; \
        exit 1; \
    fi