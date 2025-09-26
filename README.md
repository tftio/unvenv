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

### Quick Install (Recommended)

Install the latest release directly from GitHub:

```bash
curl -fsSL https://raw.githubusercontent.com/workhelix/unvenv/main/install.sh | sh
```

Or with a custom install directory:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/workhelix/unvenv/main/install.sh | sh
```

The install script will:
- Auto-detect your OS and architecture
- Download the latest release
- Verify checksums (when available)
- Install to `$HOME/.local/bin` by default
- Prompt before replacing existing installations
- Guide you on adding the directory to your PATH

### Alternative Install Methods

**From Source (requires Rust toolchain):**

```bash
git clone https://github.com/workhelix/unvenv.git
cd unvenv
cargo build --release
install -m 0755 target/release/unvenv ~/.local/bin/
```

**From Releases:**

1. Visit [Releases](https://github.com/workhelix/unvenv/releases)
2. Download the appropriate `unvenv-{target}.zip` for your platform
3. Extract and copy the binary to a directory in your PATH

### Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

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

**Automated Release Process** - This project uses `versioneer` for atomic version management:

#### Required Tools
- **`versioneer`**: Synchronizes versions across Cargo.toml and VERSION files
- **`peter-hook`**: Git hooks enforce version consistency validation
- **Automated release script**: `./scripts/release.sh` handles complete release workflow

#### Version Management Rules
1. **NEVER manually edit Cargo.toml version** - Use versioneer instead
2. **NEVER create git tags manually** - Use `versioneer tag` or release script
3. **ALWAYS use automated release workflow** - Prevents version/tag mismatches

#### Release Commands
```bash
# Automated release (recommended)
./scripts/release.sh patch   # 1.0.10 -> 1.0.11
./scripts/release.sh minor   # 1.0.10 -> 1.1.0
./scripts/release.sh major   # 1.0.10 -> 2.0.0

# Manual version management (advanced)
versioneer patch             # Bump version
versioneer sync              # Synchronize version files
versioneer verify            # Check version consistency
versioneer tag               # Create matching git tag

# Legacy just commands (deprecated)
just version-show            # Show current version
just version-patch           # Use ./scripts/release.sh patch instead
just version-minor           # Use ./scripts/release.sh minor instead
just version-major           # Use ./scripts/release.sh major instead
```

#### Quality Gates
- **Pre-push hooks**: Verify version file synchronization and tag consistency
- **GitHub Actions**: Validate tag version matches Cargo.toml before release
- **Binary verification**: Confirm built binary reports expected version
- **Release script**: Runs full quality pipeline (tests, lints, audits) before release

#### Troubleshooting
- **Version mismatch errors**: Run `versioneer verify` and `versioneer sync`
- **Tag conflicts**: Use `versioneer tag` instead of `git tag`
- **Failed releases**: Check GitHub Actions logs for version validation errors

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