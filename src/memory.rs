use helpers;
use std::collections::HashMap;
use std::ops::Deref;

static MEMORY_FILE: &'static str = "/proc/meminfo";


pub fn read_usage() -> Result<String, String> {
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

            Ok((*stats.get(&"MemTotal").unwrap()).to_string())
        }
    }
}
