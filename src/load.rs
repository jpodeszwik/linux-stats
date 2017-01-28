use helpers;

static LOAD_FILE: &'static str = "/proc/loadavg";

#[derive(RustcEncodable, Debug)]
pub struct LoadInfo {
    one: f64,
    five: f64,
    fifteen: f64
}

pub fn load() -> Result<LoadInfo, String> {
    helpers::read_file_to_string(LOAD_FILE)
        .map(|res: String| {
            let parts: Vec<&str> = res.as_str().split_whitespace().collect();
            LoadInfo {
                one: parts[0].parse::<f64>().unwrap_or(-1.0),
                five: parts[1].parse::<f64>().unwrap_or(-1.0),
                fifteen: parts[2].parse::<f64>().unwrap_or(-1.0)
            }
        })
}
