mod memory;
mod helpers;
mod temperature;


fn main() {
    let temp = temperature::temperature();
    match temp {
        None => println!("Could not read temperature"),
        _ => println!("Temperature: {}", temp.unwrap())
    }

    let mem = memory::read_usage();
    match mem {
        None => println!("Could not read memory"),
        _ => println!("{}", mem.unwrap()),
    }
}
