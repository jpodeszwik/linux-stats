use std::fmt;
use helpers;

static TEMPERATURE_FILE: &'static str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(RustcEncodable)]
pub struct TemperatureInfo {
    degree: i32,
    unit: String
}

impl fmt::Display for TemperatureInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "degree: {}, unit: {}", self.degree, self.unit)
    }
}

pub fn temperature() -> Result<TemperatureInfo, String> {
    helpers::read_file_to_string(TEMPERATURE_FILE)
        .and_then(|res| {
            res.trim()
                .parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|num|
            TemperatureInfo { degree: num / 1000, unit: "Celsius".to_string() }
        )
}
