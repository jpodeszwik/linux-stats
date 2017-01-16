use std::fs::File;
use std::io::prelude::*;


pub fn read_file_to_string(path: &str) -> Option<String> {
    let f = File::open(path);
    if f.is_err() {
        println!("Error opening file");
        return Option::None
    }

    let mut s = String::new();
    let _ = f.unwrap().read_to_string(&mut s);
    Some(s)
}
