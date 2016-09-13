use std::io;
use std::io::Write;
use std::error::Error;
use std::result::Result;
use infrastructures::github;
use infrastructures::config;
use super::Workspace;

pub fn exec() -> Result<(), Box<Error>> {
    let org_name = prompt_for_org_name();

    let repos = try!(github::fetch_org_repos(&org_name));

    let workspace = Workspace {
        host: "github.com".to_owned(),
        organization: org_name,
        repositories: repos,
    };

    try!(config::save(&workspace));

    Ok(())
}

fn prompt_for_org_name() -> String {
    print!("GitHub organization name: ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_line(&mut buff).unwrap();
    buff.trim().to_owned()
}
