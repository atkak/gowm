use std::error::Error;
use std::result::Result;
use infrastructures::github;

mod init;

pub fn init() -> Result<(), Box<Error>> {
    init::exec()
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Workspace {
    host: String,
    organization: String,
    repositories: Vec<github::Repo>,
}
