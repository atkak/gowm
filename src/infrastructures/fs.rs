use std::io;
use std::env;

pub fn current_dir_name() -> io::Result<String> {
    let path = try!(env::current_dir());
    let dir_name = path.components()
        .last()
        .and_then({ |comp| comp.as_os_str().to_str() })
        .unwrap();

    Ok(dir_name.to_owned())
}
