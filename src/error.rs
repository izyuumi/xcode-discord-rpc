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
        match self {
            // Discord IPC failures are typically recoverable by reconnecting.
            Error::DiscordIpc(_) => true,

            // AppleScript failures are often transient (Xcode not ready / busy / not responding).
            Error::Oascript(_) => true,

            // Only treat *some* IO errors as transient; many (e.g. NotFound/PermissionDenied)
            // are persistent and should not be blindly retried.
            Error::Io(e) => matches!(
                e.kind(),
                std::io::ErrorKind::Interrupted
                    | std::io::ErrorKind::WouldBlock
                    | std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::NotConnected
                    | std::io::ErrorKind::BrokenPipe
            ),

            _ => false,
        }
    }
}
