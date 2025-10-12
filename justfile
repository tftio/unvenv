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

# Bump version (patch|minor|major)
bump-version level:
    @echo "Bumping {{ level }} version..."
    @if command -v versioneer >/dev/null 2>&1; then \
        versioneer {{ level }}; \
        echo "✅ Version bumped to: $(cat VERSION)"; \
    else \
        echo "❌ versioneer not found. Install with: cargo install versioneer"; \
        exit 1; \
    fi

# Release workflow with comprehensive validation (replaces scripts/release.sh)
release level:
    #!/usr/bin/env bash
    set -euo pipefail

    # Validate bump type
    case "{{ level }}" in
        patch|minor|major) ;;
        *)
            echo "❌ Invalid bump type: {{ level }}"
            echo "Usage: just release [patch|minor|major]"
            exit 1
            ;;
    esac

    echo "🚀 Starting release workflow for unvenv..."
    echo ""

    # Prerequisites validation
    echo "Step 1: Validating prerequisites..."
    if ! command -v versioneer >/dev/null 2>&1; then
        echo "❌ versioneer not found. Install with: cargo install versioneer"
        exit 1
    fi

    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        echo "❌ Not in a git repository"
        exit 1
    fi

    if ! git diff-index --quiet HEAD --; then
        echo "❌ Working directory is not clean. Please commit or stash changes."
        git status --short
        exit 1
    fi

    CURRENT_BRANCH=$(git branch --show-current)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        echo "❌ Must be on main branch for release (currently on: $CURRENT_BRANCH)"
        exit 1
    fi

    git fetch origin main >/dev/null 2>&1
    LOCAL=$(git rev-parse HEAD)
    REMOTE=$(git rev-parse origin/main)
    if [ "$LOCAL" != "$REMOTE" ]; then
        echo "❌ Local main branch is not up-to-date with origin/main"
        echo "Run: git pull origin main"
        exit 1
    fi

    if ! versioneer verify >/dev/null 2>&1; then
        echo "❌ Version files are not synchronized"
        echo "Run: versioneer sync"
        exit 1
    fi

    CURRENT_VERSION=$(versioneer show)
    echo "✅ Prerequisites validated (current version: $CURRENT_VERSION)"
    echo ""

    # Quality gates
    echo "Step 2: Running quality gates..."
    just test
    just audit
    just deny
    just pre-commit
    echo "✅ All quality gates passed"
    echo ""

    # Version management
    echo "Step 3: Bumping {{ level }} version..."
    versioneer {{ level }}
    NEW_VERSION=$(versioneer show)
    echo "✅ Version bumped: $CURRENT_VERSION → $NEW_VERSION"
    echo ""

    # Create commit FIRST
    echo "Step 4: Committing changes..."
    git add Cargo.toml Cargo.lock VERSION
    git commit -m "chore: bump version to $NEW_VERSION"
    echo "✅ Changes committed"
    echo ""

    # Create tag AFTER commit
    echo "Step 5: Creating git tag..."
    versioneer tag --tag-format "v{version}"
    echo "✅ Tag created: v$NEW_VERSION"
    echo ""

    # Interactive confirmation
    echo "Ready to push release:"
    echo "  Version: $NEW_VERSION"
    echo "  Tag: v$NEW_VERSION"
    echo ""

    if [ -t 0 ]; then
        read -p "Push release to GitHub? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Release preparation complete but not pushed"
            echo "To push manually: git push origin main && git push --tags"
            exit 0
        fi
    fi

    # Push to remote
    echo "Step 6: Pushing to remote..."
    git push origin main
    git push --tags
    echo "✅ Pushed to remote"
    echo ""
    echo "🎉 Release complete! Tag v$NEW_VERSION pushed."

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
quality: pre-commit pre-push

# Run pre-commit hooks (format-check + clippy-check)
pre-commit:
    @if command -v peter-hook >/dev/null 2>&1; then \
        peter-hook run pre-commit; \
    else \
        echo "❌ peter-hook not found. Install with: cargo install peter-hook"; \
        exit 1; \
    fi

# Run pre-push hooks (test-all + security-audit + version-sync-check + tag-version-check)
pre-push:
    @if command -v peter-hook >/dev/null 2>&1; then \
        peter-hook run pre-push; \
    else \
        echo "❌ peter-hook not found. Install with: cargo install peter-hook"; \
        exit 1; \
    fi

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
    @just pre-commit
    @just pre-push

# Lint code with clippy
lint:
    @just pre-commit
    @just pre-push

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
dev: format pre-commit test
    @echo "✅ Development checks complete! Ready to commit."

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