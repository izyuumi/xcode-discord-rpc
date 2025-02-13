use clap::{Arg, ArgAction, Command as ClapCommand};

const SHOW_FILE_ARG_ID: &str = "show_file";
const SHOW_PROJECT_ARG_ID: &str = "show_project";

#[derive(Copy, Clone)]
pub struct ClapFlags {
    pub show_file: bool,
    pub show_project: bool,
}

pub fn init() -> ClapFlags {
    let matches = ClapCommand::new("Xcode Discord RPC")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Displays Xcode status on Discord Rich Presence")
        .arg(
            Arg::new(SHOW_FILE_ARG_ID)
                .short('f')
                .long("show-file")
                .num_args(0)
                .action(ArgAction::SetFalse)
                .help("Hide current file in Discord Rich Presence")
                .default_value("true"),
        )
        .arg(
            Arg::new(SHOW_PROJECT_ARG_ID)
                .short('p')
                .long("show-project")
                .num_args(0)
                .action(ArgAction::SetFalse)
                .help("Hide current project in Discord Rich Presence")
                .default_value("true"),
        )
        .get_matches();
    ClapFlags {
        show_file: matches.get_flag(SHOW_FILE_ARG_ID),
        show_project: matches.get_flag(SHOW_PROJECT_ARG_ID),
    }
}
