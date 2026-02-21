# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainer or use [GitHub's private vulnerability reporting](https://github.com/izyuumi/xcode-discord-rpc/security/advisories/new)
3. Include steps to reproduce and potential impact

We'll acknowledge receipt within 48 hours and aim to release a fix within 7 days for critical issues.

## Scope

This project runs as a local macOS process. Key security considerations:

- **AppleScript execution** — The app runs AppleScript commands to query Xcode state. These are read-only queries.
- **Discord IPC** — Communication happens over a local Unix socket. No network traffic.
- **Config file** — Loaded from `~/.config/xcode-discord-rpc/config.toml`. Ensure proper file permissions.
- **No telemetry** — The app does not phone home or collect any data.
