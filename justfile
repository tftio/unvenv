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

# Release workflow - validates and publishes a git tag
release:
    #!/usr/bin/env bash
    set -euo pipefail

    PROJECT_NAME="unvenv"

    echo "🚀 Starting release workflow for $PROJECT_NAME..."
    echo ""

    # Read current version from VERSION file
    if [ ! -f VERSION ]; then
        echo "❌ VERSION file not found"
        exit 1
    fi
    CURRENT_VERSION=$(cat VERSION)
    TAG="v$CURRENT_VERSION"

    echo "📋 Release Information:"
    echo "  Project: $PROJECT_NAME"
    echo "  Version: $CURRENT_VERSION"
    echo "  Tag: $TAG"
    echo ""

    # Invariant 1: Clean repository
    echo "Step 1: Checking repository is clean..."
    if ! git diff-index --quiet HEAD --; then
        echo "❌ Working directory not clean"
        git status --short
        exit 1
    fi
    echo "✅ Repository is clean"
    echo ""

    # Invariant 2: Local and remote HEAD in sync
    echo "Step 2: Checking local and remote HEAD are in sync..."
    git fetch origin main 2>/dev/null || true
    LOCAL_HEAD=$(git rev-parse HEAD)
    REMOTE_HEAD=$(git rev-parse origin/main)
    if [ "$LOCAL_HEAD" != "$REMOTE_HEAD" ]; then
        echo "❌ Local HEAD and origin/main are not in sync"
        echo "  Local:  $LOCAL_HEAD"
        echo "  Remote: $REMOTE_HEAD"
        echo "Run: git push origin main"
        exit 1
    fi
    echo "✅ Local and remote HEAD in sync: ${LOCAL_HEAD:0:8}"
    echo ""

    # Invariant 3: No existing tag for current VERSION
    echo "Step 3: Checking tag does not exist..."
    git fetch --tags origin 2>/dev/null || true
    if git tag -l "$TAG" | grep -q "^$TAG$"; then
        echo "❌ Tag $TAG already exists locally"
        git show "$TAG" --no-patch
        exit 1
    fi
    if git ls-remote --tags origin | grep -q "refs/tags/$TAG$"; then
        echo "❌ Tag $TAG already exists on remote"
        exit 1
    fi
    echo "✅ Tag $TAG does not exist"
    echo ""

    # Invariant 4: No future version tags exist
    echo "Step 4: Checking no future version tags exist..."
    FUTURE_TAGS=$(git tag -l 'v*' | sed 's/^v//' | while read ver; do
        if [ -z "$ver" ]; then continue; fi
        # Use sort -V for semantic version comparison
        LATEST=$(printf '%s\n%s' "$CURRENT_VERSION" "$ver" | sort -V | tail -n1)
        if [ "$LATEST" = "$ver" ] && [ "$ver" != "$CURRENT_VERSION" ]; then
            echo "$ver"
        fi
    done)
    if [ -n "$FUTURE_TAGS" ]; then
        echo "❌ Future version tags exist:"
        echo "$FUTURE_TAGS" | sed 's/^/  v/'
        exit 1
    fi
    echo "✅ No future version tags found"
    echo ""

    # Invariant 5: Version consistency
    echo "Step 5: Validating version consistency..."
    CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    echo "  VERSION file: $CURRENT_VERSION"
    echo "  Cargo.toml:   $CARGO_VERSION"
    if [ "$CURRENT_VERSION" != "$CARGO_VERSION" ]; then
        echo "❌ Version mismatch between VERSION and Cargo.toml"
        exit 1
    fi
    echo "✅ Version consistency validated"
    echo ""

    # Create tag at HEAD
    echo "Step 6: Creating tag..."
    git tag -a "$TAG" -m "Release $CURRENT_VERSION"
    echo "✅ Created tag: $TAG"
    echo ""

    # Confirm before pushing
    echo "Ready to publish release:"
    echo "  Tag: $TAG"
    echo "  Version: $CURRENT_VERSION"
    echo "  Commit: ${LOCAL_HEAD:0:8}"
    echo ""

    if [ -t 0 ]; then
        read -p "Push tag to trigger release? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Release cancelled"
            echo "To push manually: git push origin $TAG"
            exit 0
        fi
    fi

    # Push tag
    echo "Step 7: Pushing tag to remote..."
    git push origin "$TAG"
    echo "✅ Tag pushed to remote"
    echo ""
    echo "🎉 Release $TAG published!"
    echo ""
    echo "GitHub Actions will now:"
    echo "  1. Create draft release"
    echo "  2. Build cross-platform binaries"
    echo "  3. Publish release"
    echo ""
    echo "Monitor progress: gh run list --workflow=release.yml"

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