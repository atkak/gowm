#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate rustc_serialize;
extern crate core;
extern crate toml;
extern crate clap;

use std::env;
use std::io;
use std::fs::File;
use std::error::Error;
use std::result::Result;
use hyper::client::Client;
use hyper::header::UserAgent;
use hyper::status::StatusCode;
use rustc_serialize::json;
use clap::{Arg, App, SubCommand};


fn main() {
    env_logger::init().unwrap();


    if let Err(error) = run() {
        error!("Failed. error: {:?}", error);
        std::process::exit(1);
    }
}

enum Command {
    Init,
}

fn extract_args() -> Result<Command, String> {
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

fn run() -> Result<(), Box<Error>> {
    let command = try!(extract_args());

    match command {
        Command::Init => init(),
    }
}

fn init() -> Result<(), Box<Error>> {
    print!("GitHub organization name: ");
    io::stdout().flush().unwrap();

    let org_name = {
        let stdin = io::stdin();
        let mut buff = String::new();
        stdin.read_line(&mut buff).unwrap();
        buff.trim().to_owned()
    };

    let ref dir_name = try!(extract_dir_name());

    let repos = try!(fetch(&org_name));
    let workspace = Workspace {
        host: "github.com".to_owned(),
        organization: org_name,
        repositories: repos,
    };

    let toml_str = toml::encode_str(&workspace);

    use std::io::Write;
    use std::io::BufWriter;
    let mut file = BufWriter::new(try!(File::create(".gowm")));
    try!(file.write_all(toml_str.as_bytes()));

    Ok(())
}

#[derive(Debug)]
struct GeneralError {
    message: String,
}

use std::fmt;
impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for GeneralError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<String> for GeneralError {
    fn from(str: String) -> Self {
        GeneralError { message: str }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Repo {
    name: String,
    html_url: String,
    git_url: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Workspace {
    host: String,
    organization: String,
    repositories: Vec<Repo>,
}

fn fetch(org: &str) -> Result<Vec<Repo>, String> {
    let client = Client::new();
    let ref url = format!("https://api.github.com/orgs/{}/repos", org);
    let mut res = client.get(url)
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
