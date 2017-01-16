mod memory;
mod helpers;
mod temperature;


fn main() {
    let temp = temperature::temperature();
    match temp {
        Err(err) => println!("Could not read temperature: {}", err),
        Ok(val) => println!("Temperature: {}", val)
    }

    let mem = memory::read_usage();
    match mem {
        None => println!("Could not read memory"),
        _ => println!("{}", mem.unwrap()),
    }
}
