use discord_rich_presence::DiscordIpc;
use simple_logger::SimpleLogger;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

    // Set up graceful shutdown on SIGINT / SIGTERM
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        log::info!("Received shutdown signal, cleaning up...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set signal handler");

    log::info!("Starting xcode-discord-rpc");

    while running.load(Ordering::SeqCst) {
        match discord_rpc(&config, &running) {
            Ok(()) => {} // clean shutdown
            Err(err) => {
                log::error!("{}", err);
                log::debug!("Trying to reconnect...");
                sleep(config.update_interval);
            }
        }
        if running.load(Ordering::SeqCst) {
            sleep(config.update_interval);
        }
    }

    log::info!("Exited cleanly");
    Ok(())
}

fn discord_rpc(config: &AppConfig, running: &Arc<AtomicBool>) -> Result<()> {
    let mut client = init_discord_ipc()?;

    let mut xcode_state = XcodeState::new(config, &mut client);

    xcode_state.run(running)?;

    // Clear presence before disconnecting
    if let Err(e) = client.clear_activity() {
        log::debug!("Failed to clear Discord activity: {}", e);
    }
    if let Err(e) = client.close() {
        log::debug!("Failed to close Discord IPC: {}", e);
    }

    Ok(())
}
