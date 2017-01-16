use helpers;
use std::collections::HashMap;
use std::ops::Deref;

static MEMORY_FILE: &'static str = "/proc/meminfo";

fn str_to_string(s: &str) -> String {
    s.to_string()
}

pub fn read_usage() -> Result<String, String> {
    let content = helpers::read_file_to_string(MEMORY_FILE);
    match content {
        Err(err) => Err(err),
        Ok(s) => {
            let lines: Vec<String> = s.split('\n').map(str_to_string).collect();
            let mut stats: HashMap<String, String> = HashMap::new();
            for line in lines {
                let parts: Vec<String> = line.split_whitespace().map(str_to_string).collect();
                if parts.len() >= 2 {
                    let parts_deref = parts.deref();
                    let first: String = parts_deref[0].clone();
                    let second: String = parts_deref[1].clone();
                    stats.insert(first, second);
                }
            }

            Ok((*stats.get(&"MemTotal:".to_string()).unwrap()).clone())
        }
    }
}
