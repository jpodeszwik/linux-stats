use helpers;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

static MEMORY_FILE: &'static str = "/proc/meminfo";

pub struct MemoryInfo {
    mem_total: String,
    mem_free: String,
    mem_available: String,
    buffers: String,
    cached: String
}

impl fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mem_total: {}, mem_free: {}, mem_available: {}, buffers: {}, cached: {}",
               self.mem_total, self.mem_free, self.mem_available, self.buffers, self.cached)
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
                mem_total: (stats.get(&"MemTotal").map(|s| *s).unwrap_or("0 kB")).to_string(),
                mem_free: (stats.get(&"MemFree").map(|s| *s).unwrap_or("0 kB")).to_string(),
                mem_available: (stats.get(&"MemAvailable").map(|s| *s).unwrap_or("0 kB")).to_string(),
                buffers: (stats.get(&"Buffers").map(|s| *s).unwrap_or("0 kB")).to_string(),
                cached: (stats.get(&"Cached").map(|s| *s).unwrap_or("0 kB")).to_string()
            };

            Ok(info)
        }
    }
}
