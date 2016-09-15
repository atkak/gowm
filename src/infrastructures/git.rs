use std::io;
use std::process::Command;

pub fn clone(git_url: &str) -> io::Result<()> {
    let args = &["clone", git_url];
    let git = Command::new("git")
        .args(args)
        .spawn()
        .expect("Fail to execute command");

    try!(git.wait_with_output());

    Ok(())
}
