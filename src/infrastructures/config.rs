use std::io;
use std::io::{Write, BufWriter};
use std::fs::File;
use toml;
use apps::workspace::Workspace;

pub fn save(workspace: &Workspace) -> io::Result<()> {
    let toml_str = toml::encode_str(workspace);

    let mut file = BufWriter::new(try!(File::create(".gowm")));
    try!(file.write_all(toml_str.as_bytes()));

    Ok(())
}
