pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Discord IPC error: {0}")]
    DiscordIpc(String),
    #[error("AppleScript error: {0}")]
    Oascript(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    BoxDyn(#[from] Box<dyn std::error::Error>),
    #[error("Logger error: {0}")]
    SimpleLogger(#[from] log::SetLoggerError),
}

impl Error {
    /// Returns true if the error is transient and may resolve on retry
    /// (e.g. Discord IPC disconnects, AppleScript timeouts, IO failures).
    pub fn is_transient(&self) -> bool {
        matches!(self, Error::DiscordIpc(_) | Error::Oascript(_) | Error::Io(_))
    }
}
