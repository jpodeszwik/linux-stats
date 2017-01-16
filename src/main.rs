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
        Err(err) => println!("Could not read memory: {}", err),
        Ok(val) => println!("{}", val),
    }
}
