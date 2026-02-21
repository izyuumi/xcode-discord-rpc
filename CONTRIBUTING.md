# Contributing

Thanks for wanting to contribute!

## Setup

```bash
git clone https://github.com/izyuumi/xcode-discord-rpc.git
cd xcode-discord-rpc
cargo build
```

### Requirements
- macOS (required for AppleScript / Xcode integration)
- Rust stable
- Discord running (for testing Rich Presence)
- Xcode installed (for live testing)

## Running

```bash
cargo run          # Release logging (info)
RUST_LOG=debug cargo run  # Verbose logging
```

## Project Structure

```
src/
├── main.rs              # Entry point, reconnect loop
├── config.rs            # CLI args + config file + env vars
├── error.rs             # Error types (thiserror)
├── xcode_state.rs       # Main loop: Xcode monitoring + Discord updates
└── utils/
    ├── mod.rs           # Discord IPC init, sleep, time helpers
    ├── osascript.rs     # AppleScript queries (file, project, frontmost)
    └── file_language.rs # File extension → language/icon mapping
```

## Configuration

See [docs/config.md](docs/config.md) for all options.

Priority: CLI flags > config file > environment variables > defaults.

## Adding a New File Language

1. Add a variant to `FileLanguage` enum in `src/utils/file_language.rs`
2. Add asset keys in `get_asset_keys()`
3. Add extension mapping in the `ToFileLanguage` impl for `str`
4. Upload the icon to the Discord Developer Portal app assets

## Code Style

- `cargo fmt` before committing
- `cargo clippy -- -D warnings` should pass
- Conventional commits: `feat:`, `fix:`, `docs:`, `chore:`

## Pull Requests

1. Branch off `main`
2. One feature/fix per PR
3. Test with Discord + Xcode running
4. Describe what and why
