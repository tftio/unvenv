# Changelog

All notable changes to unvenv will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.7] - 2025-09-23

### Changed
- Modified scan behavior to check entire current directory instead of only Git-tracked files
- Git repository is now optional - tool works outside Git repositories
- Still respects .gitignore rules when Git repository is available
- Improved directory scanning to be independent of Git working directory detection

## [0.1.0] - 2025-09-23

### Added
- Initial release
- Core functionality implementation
- Basic CLI interface
- Documentation and README
- MIT License
- GitHub Actions CI/CD
- Cross-platform support

[Unreleased]: https://github.com/yourusername/unvenv/compare/v1.0.7...HEAD
[1.0.7]: https://github.com/yourusername/unvenv/compare/v0.1.0...v1.0.7
[0.1.0]: https://github.com/yourusername/unvenv/releases/tag/v0.1.0