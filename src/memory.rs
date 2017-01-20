use helpers;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

static MEMORY_FILE: &'static str = "/proc/meminfo";

#[derive(RustcEncodable)]
pub struct MemoryInfo {
    mem_total: i64,
    mem_free: i64,
    mem_available: i64,
    buffers: i64,
    cached: i64
}

impl fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mem_total: {}, mem_free: {}, mem_available: {}, buffers: {}, cached: {}",
               self.mem_total, self.mem_free, self.mem_available, self.buffers, self.cached)
    }
}

trait GetSize {
    fn get_size(&self, &str) -> i64;
}

impl<'a> GetSize for HashMap<&'a str, &'a str> {
    fn get_size(&self, key: &str) -> i64 {
        self.get(&key)
            .and_then(|s| s.split(' ').next())
            .map(|s| s.parse::<i64>().unwrap_or(0_i64))
            .unwrap_or(0_i64) * 1024
    }
}

pub fn read_usage() -> Result<MemoryInfo, String> {
    let content = helpers::read_file_to_string(MEMORY_FILE);
    match content {
        Err(err) => Err(err),
        Ok(s) => {
            let lines: Vec<&str> = s.split('\n').collect();
            let mut stats: HashMap<&str, &str> = HashMap::new();
            for line in lines {
                let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
                let parts = parts.deref();
                if parts.len() >= 2 {
                    stats.insert(parts[0], parts[1]);
                }
            }

            let info = MemoryInfo {
                mem_total: stats.get_size("MemTotal"),
                mem_free: stats.get_size("MemFree"),
                mem_available: stats.get_size("MemAvailable"),
                buffers: stats.get_size("Buffers"),
                cached: stats.get_size("Cached")
            };

            Ok(info)
        }
    }
}
