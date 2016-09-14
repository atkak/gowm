use std::io;
use std::io::Write;

pub fn prompt_for_org_name() -> String {
    print!("GitHub organization name: ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_line(&mut buff).unwrap();
    buff.trim().to_owned()
}
