use std::process::Command;
use std::str;


pub fn read_usage() -> Option<String> {
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
