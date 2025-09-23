# unvenv

[![CI](https://github.com/yourusername/unvenv/workflows/CI/badge.svg)](https://github.com/yourusername/unvenv/actions/workflows/ci.yml)
[![Release](https://github.com/yourusername/unvenv/workflows/Release/badge.svg)](https://github.com/yourusername/unvenv/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A CLI tool written in Rust

## Features

- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Fast & Reliable**: Built with Rust for maximum performance
- **Professional Quality**: Comprehensive CI/CD, linting, and testing
- **CLI Interface**: Full-featured command-line interface with help and validation
- **Colorized Output**: Beautiful terminal output with colors
- **Progress Indicators**: Visual progress bars for long-running operations

## Installation

### From Source

```bash
git clone https://github.com/yourusername/unvenv.git
cd unvenv
cargo install --path .
```

### From Releases

Download the latest binary for your platform from the [Releases](https://github.com/yourusername/unvenv/releases) page.

## Usage

### Basic Usage

```bash
# Get help
unvenv --help

# Get version
unvenv --version

# Process input
unvenv "your-input-here"

# Verbose output
unvenv --verbose "your-input-here"
```

### Examples

```bash
# Example 1: Basic processing
unvenv "example input"

# Example 2: Verbose mode
unvenv --verbose "example input"
```

## Configuration

Configuration files are stored in:
- **Linux/macOS**: `~/.config/unvenv/`
- **Windows**: `%APPDATA%\unvenv\`

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (MSRV: 1.70.0)
- [Git](https://git-scm.com/)
- [just](https://github.com/casey/just) (recommended for development)
- [peter-hook](https://github.com/example/peter-hook) (for git hooks)
- [versioneer](https://github.com/example/versioneer) (for version management)

### Quick Start with Just

```bash
# Clone the repository
git clone https://github.com/yourusername/unvenv.git
cd unvenv

# Setup development environment
just setup

# Build the project
just build

# Run tests
just test

# Development workflow (format + lint + test)
just dev

# Show all available commands
just
```

### Manual Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with development profile
cargo run -- --help
```

### Code Quality

This project maintains high code quality standards:

#### Using Just (Recommended)
```bash
# Format code
just format

# Check formatting
just format-check

# Lint code
just lint

# Security audit
just audit

# Dependency compliance
just deny

# All quality checks
just quality

# Full CI pipeline
just ci
```

#### Manual Commands
```bash
# Format code (requires nightly rustfmt)
cargo +nightly fmt

# Lint code
cargo clippy -- -D warnings

# Security audit
cargo audit

# Dependency compliance
cargo deny check
```

### Version Management

Use the `just` commands for version bumping:

```bash
# Show current version
just version-show

# Bump patch version (0.1.0 -> 0.1.1)
just version-patch

# Bump minor version (0.1.0 -> 0.2.0)
just version-minor

# Bump major version (0.1.0 -> 1.0.0)
just version-major
```

### Git Hooks

This project uses [peter-hook](https://github.com/example/peter-hook) for git hooks:

```bash
# Install hooks
just install-hooks

# Or manually (if peter-hook is installed)
peter-hook install
```

## Architecture

### Project Structure

```
unvenv/
├── .github/          # GitHub Actions workflows
│   ├── workflows/    # CI/CD pipelines
│   └── dependabot.yml
├── src/              # Source code
│   └── main.rs       # Application entry point
├── tests/            # Integration tests
├── Cargo.toml        # Project metadata and dependencies
├── deny.toml         # Dependency compliance rules
├── rustfmt.toml      # Code formatting configuration
├── clippy.toml       # Linting configuration
├── hooks.toml        # Git hooks configuration
└── justfile          # Development workflow automation
```

### Key Components
- **CLI Interface**: Built with [clap](https://docs.rs/clap/) for robust argument parsing
- **Serialization**: Uses [serde](https://docs.rs/serde/) for data serialization/deserialization
- **Error Handling**: Leverages [anyhow](https://docs.rs/anyhow/) for ergonomic error handling
- **Terminal Output**: Enhanced with [colored](https://docs.rs/colored/) for rich terminal display
- **Progress Display**: Integrated [indicatif](https://docs.rs/indicatif/) for progress indicators

## CI/CD

This project includes comprehensive CI/CD pipelines:

- **Continuous Integration**: Format, lint, test, audit, and build on multiple platforms
- **Release Automation**: Automated releases with cross-platform binaries
- **Dependency Management**: Automated dependency updates via Dependabot
- **Security Scanning**: Regular security audits and vulnerability checks

### Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: Intel (x86_64), Apple Silicon (aarch64)
- **Windows**: x86_64

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run quality checks (`cargo fmt`, `cargo clippy`, `cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/unvenv/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/unvenv/discussions)
- **Email**: your.email@example.com

---

**unvenv** - A CLI tool written in Rust