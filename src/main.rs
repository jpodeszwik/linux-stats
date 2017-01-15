use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;

fn read_file_to_string(path: String) -> Option<String> {
    let f = File::open(path);
    if f.is_err() {
        println!("Error opening file");
        return Option::None
    }

    let mut s = String::new();
    let _ = f.unwrap().read_to_string(&mut s);
    Some(s)
}

fn read_memory_usage() -> Option<String> {
    let output = Command::new("free")
        .arg("-m")
        .output();

    if output.is_err() {
        return Option::None
    }

    let stdout = output.unwrap().stdout;
    let output_str = str::from_utf8(&stdout);

    if output_str.is_err() {
        return Option::None
    }

    Some(output_str.unwrap().to_string())
}

fn main() {
    let res = read_file_to_string("/sys/class/thermal/thermal_zone0/temp".to_string());
    match res {
        None => println!("Could not read temperature"),
        _ => println!("{}", res.unwrap()),
    }

    let mem = read_memory_usage();
    match mem {
        None => println!("Could not read memory"),
        _ => println!("{}", mem.unwrap()),
    }
}
