use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute osascript: {0}")]
    Osascript(String),
    #[error("Invalid UTF-8 in osascript output")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("System time error: {0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error("Simple logger error: {0}")]
    SimpleLogger(#[from] log::SetLoggerError),
    #[error("Box<dyn Error> error: {0}")]
    BoxDyn(#[from] Box<dyn std::error::Error>),
}
