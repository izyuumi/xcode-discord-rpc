pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Discord IPC error: {0}")]
    DiscordIpc(String),
    #[error("AppleScript error: {0}")]
    Oascript(String),
    #[error("{0}")]
    BoxDyn(#[from] Box<dyn std::error::Error>),
    #[error("Logger init error: {0}")]
    SimpleLogger(#[from] log::SetLoggerError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Whether this error is transient and the operation should be retried.
    pub fn is_transient(&self) -> bool {
        matches!(self, Error::DiscordIpc(_) | Error::Oascript(_) | Error::Io(_))
    }
}
