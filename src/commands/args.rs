use std::result::Result;
use clap::{App, SubCommand};

pub enum Command {
    Init,
    Open,
}

pub fn extract_args() -> Result<Command, String> {
    let matches = App::new("gowm")
        .version("0.1.0")
        .about("Workspace manager for each GitHub organizations.")
        .subcommand(SubCommand::with_name("init")
            .about("Initialize current directory as workspace."))
        .subcommand(SubCommand::with_name("open")
            .about("Open selected project in the current workspace."))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(_)) => Ok(Command::Init),
        ("open", Some(_)) => Ok(Command::Open),
        _ => Err(matches.usage().to_owned()),
    }
}
