use std::error::Error;
use std::result::Result;
use super::Workspace;

pub fn exec() -> Result<(), Box<Error>> {
    let mut workspace = Workspace::create_new();

    println!("Fetch organization repositories");

    try!(workspace.fetch_repos());

    println!("Create workspace");

    try!(workspace.save());

    println!("Done");

    Ok(())
}
