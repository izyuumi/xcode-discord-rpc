pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("DiscordIPC error: {0}")]
    DiscordIpc(String),
    #[error("Oascript error: {0}")]
    Oascript(String),
    #[error("BoxDyn error: {0}")]
    BoxDyn(#[from] Box<dyn std::error::Error>),
    #[error("Custom error: {0}")]
    Custom(String),
}
