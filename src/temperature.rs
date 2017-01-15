use helpers;

pub fn temperature() -> Option<i32> {
    let res = helpers::read_file_to_string("/sys/class/thermal/thermal_zone0/temp".to_string());
    match res {
        None => {
            println!("Could not read temperature");
            Option::None
        },
        _ => {
            let mut str_res = res.unwrap();
            str_res.pop();
            let num = str_res.parse::<i32>();
            if num.is_err() {
                println!("Error decoding int");
                return Option::None
            }

            return Option::Some(num.unwrap() / 1000)
        },
    }
}
