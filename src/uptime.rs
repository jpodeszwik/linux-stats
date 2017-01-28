use helpers;

static UPTIME_FILE: &'static str = "/proc/uptime";

#[derive(RustcEncodable, Debug)]
pub struct Uptime {
    uptime: String
}

pub fn uptime() -> Result<Uptime, String> {
    let content = helpers::read_file_to_string(UPTIME_FILE);

    match content {
        Err(err) => Err(err),
        Ok(val) => Ok(Uptime { uptime: val.trim().to_string() })
    }
}