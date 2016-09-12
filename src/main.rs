#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate rustc_serialize;
extern crate core;
extern crate toml;

use std::env;
use std::io;
use std::fs::File;
use std::error::Error;
use std::result::Result;
use hyper::client::Client;
use hyper::header::UserAgent;
use hyper::status::StatusCode;
use rustc_serialize::json;


fn main() {
    env_logger::init().unwrap();

    if let Err(error) = run() {
        error!("Failed. error: {:?}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let ref dir_name = try!(extract_dir_name().map_err(CliError::IO));

    let repos = Repos { repositories: try!(fetch(dir_name).map_err(CliError::General)) };
    let toml_str = toml::encode_str(&repos);

    use std::io::Write;
    use std::io::BufWriter;
    let mut file = BufWriter::new(try!(File::create(".gowm").map_err(CliError::IO)));
    try!(file.write_all(toml_str.as_bytes()).map_err(CliError::IO));

    Ok(())
}

#[derive(Debug)]
enum CliError {
    General(String),
    IO(io::Error),
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Repo {
    name: String,
    html_url: String,
    git_url: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Repos {
    repositories: Vec<Repo>
}

fn fetch(org: &str) -> Result<Vec<Repo>, String> {
    let client = Client::new();
    let mut res = client.get(format!("https://api.github.com/orgs/{}/repos", org))
        .header(UserAgent("gowm".to_owned()))
        .send()
        .unwrap();

    let ref body = try!(
        match res.status {
            StatusCode::Ok => {
                let mut body = String::new();
                use std::io::Read;
                res.read_to_string(&mut body);
                Ok(body)
            },
            StatusCode::NotFound => Err(format!("Organization not found. name: {}", org)),
            _ => Err("Fail to access to GitHub".to_owned())
        }
    );

    let repos: Vec<Repo> = try!(
        json::decode(body).map_err(|err| err.description().to_owned())
    );

    Ok(repos)
}

fn extract_dir_name() -> io::Result<String> {
    let path = try!(env::current_dir());
    let dir_name = path.components()
        .last()
        .and_then({ |comp| comp.as_os_str().to_str() })
        .unwrap();

    Ok(dir_name.to_owned())
}
