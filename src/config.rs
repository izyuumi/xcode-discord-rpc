use std::time::Duration;

pub mod default {
    use std::time::Duration;

    #[cfg(debug_assertions)]
    pub const UPDATE_INTERVAL: Duration = Duration::from_secs(3);
    #[cfg(not(debug_assertions))]
    pub const UPDATE_INTERVAL: Duration = Duration::from_secs(30);

    #[cfg(debug_assertions)]
    pub const XCODE_CHECK_CYCLE: i8 = 1;
    #[cfg(not(debug_assertions))]
    pub const XCODE_CHECK_CYCLE: i8 = 5;

    pub const IDLE_THRESHOLD: Duration = Duration::from_secs(10);
}

#[derive(Debug)]
pub struct Config {
    show_file: bool,
    show_project: bool,
    client_id: String,
    update_interval: Duration,
    idle_threshold: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_file: true,
            show_project: true,
            client_id: String::from("1158013054898950185"),
            update_interval: default::UPDATE_INTERVAL,
            idle_threshold: default::IDLE_THRESHOLD,
        }
    }
}
