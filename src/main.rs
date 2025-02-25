use simple_logger::SimpleLogger;

mod config;
mod error;
mod utils;
mod xcode_state;

use config::AppConfig;
#[allow(unused)]
pub use error::{Error, Result};
use utils::{init_discord_ipc, sleep};
use xcode_state::XcodeState;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;

    #[cfg(not(debug_assertions))]
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let config = AppConfig::new()?;

    log::info!("Starting xcode-discord-rpc");

    loop {
        if let Err(err) = discord_rpc(&config) {
            log::error!("{}", err);
            log::debug!("Trying to reconnect...");
            sleep(config.update_interval)
        }
        sleep(config.update_interval)
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn discord_rpc(config: &AppConfig) -> Result<()> {
    let mut client = init_discord_ipc()?;

    let mut xcode_state = XcodeState::new(config, &mut client);

    xcode_state.run()?;

    #[allow(unreachable_code)]
    Ok(())
}
