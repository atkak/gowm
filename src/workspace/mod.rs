use std;
use std::error::Error;
use std::result::Result;
use infrastructures::github;
use infrastructures::git;

mod init;
mod open;
mod config;
mod io;

pub fn init() -> Result<(), Box<Error>> {
    init::exec()
}

pub fn open() -> Result<(), Box<Error>> {
    open::exec()
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

    fn load() -> std::io::Result<Workspace> {
        config::load()
    }

    fn update_repos_meta(&mut self) -> Result<(), Box<Error>> {
        self.repositories = try!(github::fetch_org_repos(self.organization.as_ref()));
        Ok(())
    }

    fn select_repo(&self) -> std::io::Result<&github::Repo> {
        let repo_names: Vec<&str> = self.repositories
            .iter()
            .map(|repo| repo.name())
            .collect();
        let target_repo_name = try!(io::prompt_for_repository_selection(&repo_names));

        let target_repo = self.repositories.iter()
        .find(|repo| repo.name() == target_repo_name)
        .unwrap();

        Ok(target_repo)
    }

    fn fetch_repo(&self, repo: &github::Repo) -> std::io::Result<()> {
        git::clone(repo.git_url())
    }

    fn save(&self) -> std::io::Result<()> {
        config::save(self)
    }
}
