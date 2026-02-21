# Changelog

All notable changes to this project will be documented in this file.

## [1.3.2] - 2026-01-21

### Fixed
- Use documented config path `~/.config/xcode-discord-rpc/config.toml`

## [1.3.1] - 2025-12-05

### Fixed
- Idle detection hang bug — idle checks were not returning control to the main loop

## [1.3.0] - 2025-10-10

### Added
- `disable_idle` config option to prevent idle status detection

### Changed
- Updated Homebrew installation/uninstallation docs

## [1.2.0] - 2025-09-18

### Added
- Exponential backoff for sleep when Xcode/Discord not running
- Reduced CPU usage when idle

## [1.1.0] - 2025-02-28

### Added
- Environment variable configuration with `XDRPC__` prefix
- Configuration file support at `~/.config/xcode-discord-rpc/config.toml`

## [1.0.0] - 2025-02-25

### Added
- Initial stable release
- Discord Rich Presence showing current file, project, and language
- Idle detection based on Xcode foreground status
- CLI flags for hiding file (`-f`) and project (`-p`)
- Support for Swift, C++, C, Objective-C, Ruby, Java, JSON, Metal
