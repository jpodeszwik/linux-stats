mod memory;
mod helpers;


fn main() {
    let res = helpers::read_file_to_string("/sys/class/thermal/thermal_zone0/temp".to_string());
    match res {
        None => println!("Could not read temperature"),
        _ => println!("{}", res.unwrap()),
    }

    let mem = memory::read_usage();
    match mem {
        None => println!("Could not read memory"),
        _ => println!("{}", mem.unwrap()),
    }
}
