use helpers;

static TEMPERATURE_FILE: &'static str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(RustcEncodable, Debug)]
pub struct TemperatureInfo {
    degree: i32,
    unit: String
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
