use std::io;
use std::io::{Write, Read, BufWriter, BufReader};
use std::fs::File;
use toml;
use super::Workspace;

pub fn save(workspace: &Workspace) -> io::Result<()> {
    let toml_str = toml::encode_str(workspace);

    let mut file = BufWriter::new(try!(File::create(".gowm")));
    try!(file.write_all(toml_str.as_bytes()));

    Ok(())
}

pub fn load() -> io::Result<Workspace> {
    let mut file = BufReader::new(try!(File::open(".gowm")));
    let mut toml_str = String::new();
    try!(file.read_to_string(&mut toml_str));

    let workspace: Workspace = toml::decode_str::<Workspace>(&toml_str).unwrap();
    Ok(workspace)
}
