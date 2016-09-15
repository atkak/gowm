use std::error::Error;
use std::result::Result;
use super::Workspace;

pub fn exec() -> Result<(), Box<Error>> {
    let mut workspace = Workspace::create_new();

    println!("Fetch organization repositories");

    try!(workspace.update_repos_meta());

    println!("Create workspace");

    try!(workspace.save());

    println!("Done");

    Ok(())
}
