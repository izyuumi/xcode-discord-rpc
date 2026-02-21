# Configuration

`xcode-discord-rpc` can be configured through command-line arguments or a configuration file, and changes to the file require an application restart. The priority is as follows: command-line arguments > configuration file > default values.

## Configuration File

You can create a configuration file at `~/.config/xcode-discord-rpc/config.toml` to override the default settings.

Example `config.toml`:

```toml
update_interval = 60
xcode_update_interval = 5
xcode_check_cycle = 10
idle_threshold = 300
disable_idle = false
hide_file = true
hide_project = false
```

## Command-Line Arguments

Command-line arguments are available for common options:

- `-f`, `--hide-file`: Hide the current file in Discord Rich Presence.
- `-p`, `--hide-project`: Hide the current project in Discord Rich Presence.
- `-i`, `--disable-idle`: Disable idle status detection.
- `-V`, `--version`: Print version information.
- `-h`, `--help`: Print help information.

## Configuration Options

### `update_interval`

- **Description**: The interval in seconds for checking and updating the Discord Rich Presence status.
- **Default**: `30`

### `xcode_update_interval`

- **Description**: The interval in seconds for checking for updates within Xcode (e.g., file changes, project changes).
- **Default**: `3`

### `xcode_check_cycle`

- **Description**: The number of update cycles to wait before re-checking if Xcode is running, especially after it was found to be closed.
- **Default**: `5`

### `idle_threshold`

- **Description**: The threshold in seconds to consider the user as idle. If there is no activity for this duration, the status will show as idle.
- **Default**: `25`

### `disable_idle`

- **Description**: A boolean value to determine whether to disable idle status detection. When set to `true`, the Discord Rich Presence will never show an idle status, even when Xcode is not in the foreground.
- **Default**: `false`
- **Command-Line Flag**: `--disable-idle` or `-i`

### `hide_file`

- **Description**: A boolean value to determine whether to hide the file name in the Discord Rich Presence.
- **Default**: `false`
- **Command-Line Flag**: `--hide-file` or `-f`

### `hide_project`

- **Description**: A boolean value to determine whether to hide the project name in the Discord Rich Presence.
- **Default**: `false`
- **Command-Line Flag**: `--hide-project` or `-p`
