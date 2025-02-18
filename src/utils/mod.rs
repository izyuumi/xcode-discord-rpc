use discord_rich_presence::DiscordIpcClient;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod file_language;
pub mod osascript;

use crate::{Error, Result};

pub fn init_discord_ipc() -> Result<DiscordIpcClient> {
    match DiscordIpcClient::new("1158013054898950185") {
        Ok(client) => Ok(client),
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
