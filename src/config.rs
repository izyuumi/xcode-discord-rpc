use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::PathBuf;

/// Argument ID for hiding the file name in Discord Rich Presence
const HIDE_FILE_ARG_ID: &str = "hide_file";
/// Argument ID for hiding the project name in Discord Rich Presence
const HIDE_PROJECT_ARG_ID: &str = "hide_project";
/// Argument ID for hiding the git branch in Discord Rich Presence
const HIDE_BRANCH_ARG_ID: &str = "hide_branch";
/// Argument ID for the custom config file path
const CONFIG_ARG_ID: &str = "config";
/// Argument ID for disabling idle detection
const DISABLE_IDLE_ARG_ID: &str = "disable_idle";
/// Content of the default configuration file
const DEFAULT_CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/default.toml"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Interval in seconds for checking status for Discord and Xcode
    pub update_interval: u64,
    /// Interval in seconds for checking updates in Xcode
    pub xcode_update_interval: u64,
    /// Number of update cycles before re-checking if Xcode is running
    pub xcode_check_cycle: u8,
    /// Threshold in seconds for considering the user idle status
    pub idle_threshold: i64,
    /// Whether to disable idle status detection
    pub disable_idle: bool,
    /// Whether to hide the file name in Discord Rich Presence
    pub hide_file: bool,
    /// Whether to hide the project name in Discord Rich Presence
    pub hide_project: bool,
    /// Whether to hide the git branch name in Discord Rich Presence
    pub hide_branch: bool,
}

impl AppConfig {
    pub fn new() -> crate::Result<Self> {
        let clap_matches = Self::get_clap_matches();

        let mut builder =
            Config::builder().add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml));

        // Config file: use --config path if provided, otherwise default location
        if let Some(config_path) = clap_matches.get_one::<String>(CONFIG_ARG_ID) {
            let path = PathBuf::from(config_path);
            log::debug!("Using custom config file: {:?}", path);
            builder = builder.add_source(File::from(path).required(true));
        } else if let Some(home) = std::env::var_os("HOME") {
            let config_path = PathBuf::from(home)
                .join(".config")
                .join("xcode-discord-rpc")
                .join("config.toml");
            log::debug!("Looking for config file at: {:?}", config_path);
            builder = builder.add_source(File::from(config_path).required(false));
        }

        let mut builder = builder
            .add_source(Environment::with_prefix("XDRPC").separator("__"));
        if clap_matches.get_flag(HIDE_FILE_ARG_ID) {
            builder = builder.set_override("hide_file", true)?;
        }
        if clap_matches.get_flag(HIDE_PROJECT_ARG_ID) {
            builder = builder.set_override("hide_project", true)?;
        }
        if clap_matches.get_flag(DISABLE_IDLE_ARG_ID) {
            builder = builder.set_override("disable_idle", true)?;
        }
        if clap_matches.get_flag(HIDE_BRANCH_ARG_ID) {
            builder = builder.set_override("hide_branch", true)?;
        }
        let c = builder.build()?;
        
        Ok(c.try_deserialize()?)
    }

    fn get_clap_matches() -> ArgMatches {
        ClapCommand::new("Xcode Discord RPC")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Displays Xcode status on Discord Rich Presence")
            .arg(
                Arg::new(CONFIG_ARG_ID)
                    .short('c')
                    .long("config")
                    .num_args(1)
                    .value_name("PATH")
                    .help("Path to a custom config file (required if specified)"),
            )
            .arg(
                Arg::new(DISABLE_IDLE_ARG_ID)
                    .short('i')
                    .long("disable-idle")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Disable idle status detection"),
            )
            .arg(
                Arg::new(HIDE_FILE_ARG_ID)
                    .short('f')
                    .long("hide-file")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Hide current file in Discord Rich Presence"),
            )
            .arg(
                Arg::new(HIDE_PROJECT_ARG_ID)
                    .short('p')
                    .long("hide-project")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Hide current project in Discord Rich Presence"),
            )
            .arg(
                Arg::new(HIDE_BRANCH_ARG_ID)
                    .short('b')
                    .long("hide-branch")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Hide current git branch in Discord Rich Presence"),
            )
            .get_matches()
    }
}
