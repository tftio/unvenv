# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

This project uses `just` as the primary task runner. All development workflows should use just commands:

### Core Development
- `just dev` - Development workflow: format + lint + test (run before commits)
- `just build` - Build in debug mode
- `just build-release` - Build in release mode
- `just test` - Run all tests with verbose output
- `just clean` - Clean build artifacts

### Code Quality
- `just format` - Format code (requires nightly rustfmt)
- `just lint` - Lint with clippy (denies warnings)
- `just audit` - Security audit with cargo-audit
- `just deny` - Dependency compliance check
- `just quality` - Run all quality checks (format-check + lint + audit)
- `just ci` - Full CI pipeline (quality + test + build-release)

### Manual Cargo Commands (fallback)
- `cargo +nightly fmt` - Format code
- `cargo clippy --all-targets -- -D warnings` - Lint code
- `cargo test --all --verbose` - Run tests
- `cargo build --release` - Release build

### Version Management
- `just version-show` - Show current version
- `just version-patch` / `just version-minor` / `just version-major` - Bump versions using versioneer

## Architecture

### Python Virtual Environment Detector
- **Purpose**: Detects Python virtual environments (`pyvenv.cfg` files) that are not ignored by Git
- **Entry Point**: `src/main.rs` - Contains the entire application
- **CLI Framework**: Uses clap v4 with subcommand pattern (version, scan)
- **Git Operations**: Uses git2 crate with vendored libgit2 for repository operations
- **File System**: Uses walkdir for efficient recursive directory traversal

### Key Dependencies
- `clap` - CLI argument parsing with derive macros and subcommands
- `anyhow` - Error handling
- `colored` - Terminal colors and styling (TTY-aware)
- `git2` - Git repository operations and ignore checking
- `walkdir` - Recursive directory traversal
- `atty` - TTY detection for output formatting

### Code Standards
- Rust edition 2021, MSRV 1.70.0
- Unsafe code forbidden via lints
- Maximum clippy strictness: all + pedantic + nursery warnings
- Missing docs warnings enabled
- Uses VERSION constant from CARGO_PKG_VERSION for version display

### Tool Behavior
- **Exit Codes**: 0 (clean/not in git repo), 1 (internal error), 2 (policy violation)
- **Subcommands**: `version` (show version), `scan` (detect venvs, default), built-in `help`
- **TTY Detection**: Automatically disables colors/decorations when output is not to terminal
- **Git Integration**: Discovers Git repository, respects .gitignore rules, skips bare repos

### Development Tools Required
- `just` - Task runner
- `peter-hook` - Git hooks management (install with `just install-hooks`)
- `versioneer` - Version management
- Rust nightly toolchain for rustfmt
- cargo-audit, cargo-deny, cargo-tarpaulin for quality checks

## Testing Strategy
Tests are in the main.rs file under #[cfg(test)]. The project includes:
- Unit tests for core functions
- Version constant validation
- Input processing tests

Use `cargo test --all --verbose` or `just test` to run all tests.