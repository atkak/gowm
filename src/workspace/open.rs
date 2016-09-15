use std::error::Error;
use std::result::Result;
use super::Workspace;

pub fn exec() -> Result<(), Box<Error>> {
    let workspace = try!(Workspace::load());

    let target_repo = try!(workspace.select_repo());

    println!("Clone {} into workspace", target_repo.name());
    println!("");

    try!(workspace.fetch_repo(target_repo));

    println!("");
    println!("Done");

    Ok(())
}
