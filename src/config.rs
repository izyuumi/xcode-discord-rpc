use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use config::{Config, Environment, File, FileFormat};
use directories::ProjectDirs;
use serde::Deserialize;

/// Argument ID for hiding the file name in Discord Rich Presence
const HIDE_FILE_ARG_ID: &str = "hide_file";
/// Argument ID for hiding the project name in Discord Rich Presence
const HIDE_PROJECT_ARG_ID: &str = "hide_project";
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
    /// Whether to hide the file name in Discord Rich Presence
    pub hide_file: bool,
    /// Whether to hide the project name in Discord Rich Presence
    pub hide_project: bool,
}

impl AppConfig {
    pub fn new() -> crate::Result<Self> {
        let clap_matches = Self::get_clap_matches();

        let mut builder =
            Config::builder().add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml));

        if let Some(proj_dirs) = ProjectDirs::from("", "", "xcode-discord-rpc") {
            let config_path = proj_dirs.config_dir().join("config.toml");
            builder = builder.add_source(File::from(config_path).required(false));
        }

        let c = builder
            .add_source(Environment::with_prefix("XDRPC").separator("__"))
            .set_override("hide_file", clap_matches.get_flag(HIDE_FILE_ARG_ID))?
            .set_override("hide_project", clap_matches.get_flag(HIDE_PROJECT_ARG_ID))?
            .build()?;

        Ok(c.try_deserialize()?)
    }

    fn get_clap_matches() -> ArgMatches {
        ClapCommand::new("Xcode Discord RPC")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Displays Xcode status on Discord Rich Presence")
            .arg(
                Arg::new(HIDE_FILE_ARG_ID)
                    .short('f')
                    .long("hide-file")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Hide current file in Discord Rich Presence")
                    .default_value("false"),
            )
            .arg(
                Arg::new(HIDE_PROJECT_ARG_ID)
                    .short('p')
                    .long("hide-project")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
                    .help("Hide current project in Discord Rich Presence")
                    .default_value("false"),
            )
            .get_matches()
    }
}
