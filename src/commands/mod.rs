use std::error::Error;
use self::args::Command;
use workspace;

mod args;

pub fn run() -> Result<(), Box<Error>> {
    let command = try!(args::extract_args());

    match command {
        Command::Init => workspace::init(),
    }
}
