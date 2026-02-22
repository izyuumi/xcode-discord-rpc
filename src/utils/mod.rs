use discord_rich_presence::DiscordIpcClient;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod file_language;
pub mod osascript;

use crate::{Error, Result};

pub fn init_discord_ipc() -> Result<DiscordIpcClient> {
    match DiscordIpcClient::new("1158013054898950185") {
        Ok(client) => {
            log::debug!("Discord IPC client initialized");
            Ok(client)
        }
        Err(err) => Err(Error::DiscordIpc(err.to_string())),
    }
}

/// Get the current time in seconds since the UNIX epoch as `i64`
pub fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to obtain current time")
        .as_secs() as i64
}

/// Sleep for `Config::update_interval` seconds
pub fn sleep(update_interval: u64) {
    std::thread::sleep(std::time::Duration::from_secs(update_interval));
}

/// Sleep with random jitter (±25%) to avoid synchronized retry storms.
/// Useful for backoff loops when Discord or Xcode is not running.
pub fn sleep_with_jitter(base_seconds: u64) {
    // Simple deterministic-ish jitter using current time nanoseconds
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let jitter_pct = (nanos % 51) as f64 / 100.0 - 0.25; // -0.25 to +0.25
    let duration = base_seconds as f64 * (1.0 + jitter_pct);
    let duration = duration.max(1.0);
    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
}
