use std::fs::File;
use std::io::prelude::*;


pub fn read_file_to_string(path: &str) -> Result<String, String> {
    File::open(path)
        .map(|mut f| {
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            s
        })
        .map_err(|err| err.to_string())
}
