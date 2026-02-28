use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        log::info!("Received shutdown signal, exiting...");
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set signal handler");

    log::info!("Starting xcode-discord-rpc");

    while running.load(Ordering::SeqCst) {
        if let Err(err) = discord_rpc(&config, &running) {
            log::error!("{}", err);
            log::debug!("Trying to reconnect...");
            sleep(config.update_interval)
        }
        sleep(config.update_interval)
    }

    log::info!("Shutting down cleanly");
    Ok(())
}

fn discord_rpc(config: &AppConfig, running: &Arc<AtomicBool>) -> Result<()> {
    let mut client = init_discord_ipc()?;

    let mut xcode_state = XcodeState::new(config, &mut client);

    xcode_state.run(running)?;

    // Clear activity before disconnecting
    if let Err(e) = xcode_state.clear_activity() {
        log::debug!("Failed to clear activity on shutdown: {}", e);
    }

    Ok(())
}
