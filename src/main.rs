#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::io;

fn main() {
    env_logger::init().unwrap();

    if let Err(error) = run() {
        error!("Failed. error: {}", error);
        std::process::exit(error.raw_os_error().unwrap_or(1));
    }
}

fn run() -> io::Result<()> {
    let dir_name = try!(extract_dir_name());
    println!("{}", dir_name);

    Ok(())
}

fn extract_dir_name() -> io::Result<String> {
    let path = try!(env::current_dir());
    let dir_name = path.components()
        .last()
        .and_then({ |comp| comp.as_os_str().to_str() })
        .unwrap();

    Ok(dir_name.to_owned())
}
