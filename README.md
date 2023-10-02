# Discord Rich Presence for Xcode

## Features

- Launched in the background at login
- No menu bar icon nor dock icon
- Presence only when Xcode is running with a project open
- Shows Swift logo when editing Swift files (more coming soon) and Xcode icon otherwise
- Written 100% in Rust

## Getting Started

### Installation with Homebrew

```bash
brew tap izyumidev/xcode-discord-rpc
brew install xcode-discord-rpc
brew services restart xcode-discord-rpc
```

If things are not working, restart Discord and/or your computer.

### Uninstallation

```bash
brew services stop xcode-discord-rpc
brew uninstall xcode-discord-rpc
brew untap izyumidev/xcode-discord-rpc
```
