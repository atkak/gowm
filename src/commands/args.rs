use std::result::Result;
use clap::{App, SubCommand};

pub enum Command {
    Init,
}

pub fn extract_args() -> Result<Command, String> {
    let matches = App::new("gowm")
        .version("0.1.0")
        .about("Workspace manager for each GitHub organizations.")
        .subcommand(SubCommand::with_name("init")
            .about("Initialize current directory as workspace."))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(_)) => Ok(Command::Init),
        _ => Err(matches.usage().to_owned()),
    }
}
