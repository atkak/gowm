use std;
use std::error::Error;
use std::result::Result;
use infrastructures::github;
use infrastructures::io;
use infrastructures::config;

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

impl Workspace {
    fn create_new() -> Workspace {
        let org_name = io::prompt_for_org_name();

        Workspace {
            host: "github.com".to_owned(),
            organization: org_name,
            repositories: vec![],
        }
    }

    fn fetch_repos(&mut self) -> Result<(), Box<Error>> {
        self.repositories = try!(github::fetch_org_repos(self.organization.as_ref()));
        Ok(())
    }

    fn save(&self) -> Result<(), std::io::Error> {
        config::save(self)
    }
}
