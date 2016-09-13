#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate rustc_serialize;
extern crate core;
extern crate toml;
extern crate clap;

mod apps;
mod commands;
mod infrastructures;
mod error;

fn main() {
    env_logger::init().unwrap();

    if let Err(error) = commands::run() {
        error!("Failed. error: {:?}", error);
        std::process::exit(1);
    }
}
