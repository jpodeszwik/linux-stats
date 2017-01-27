use std::fmt;
use helpers;

static UPTIME_FILE: &'static str = "/proc/uptime";

#[derive(RustcEncodable)]
pub struct Uptime {
    uptime: String
}

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uptime: {}", self.uptime)
    }
}

pub fn uptime() -> Result<Uptime, String> {
    let content = helpers::read_file_to_string(UPTIME_FILE);

    match content {
        Err(err) => Err(err),
        Ok(val) => Ok(Uptime { uptime: val.trim().to_string() })
    }
}