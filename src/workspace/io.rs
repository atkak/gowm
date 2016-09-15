use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn prompt_for_org_name() -> String {
    print!("GitHub organization name: ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_line(&mut buff).unwrap();
    buff.trim().to_owned()
}

pub fn prompt_for_repository_selection(repo_names: &[&str]) -> io::Result<String> {
    let mut child = Command::new("peco")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail to execute command");

    try!(write_repo_names(child.stdin.as_mut().unwrap(), repo_names));
    let output = child.wait_with_output().unwrap();

    let target_repo_name = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    Ok(target_repo_name)
}

fn write_repo_names(stdin: &mut Write, repo_names: &[&str]) -> io::Result<()> {
    for repo_name in repo_names {
        try!(writeln!(stdin, "{}", repo_name));
    }

    Ok(())
}
