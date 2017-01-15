use std::fmt;
use helpers;

pub struct TemperatureInfo {
    degree: i32,
    unit: String
}

impl fmt::Display for TemperatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "degree: {}, unit: {}", self.degree, self.unit)
    }
}

pub fn temperature() -> Option<TemperatureInfo> {
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

            return Option::Some(TemperatureInfo { degree: num.unwrap() / 1000, unit: "Celsius".to_string() })
        },
    }
}
