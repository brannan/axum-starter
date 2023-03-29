use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_env() -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open(".env")?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        // skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // skip lines that start with # or ;
        if line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        // parse values
        let (key, value) = line.split_once("=").unwrap();
        map.insert(key.trim().to_owned(), value.trim().to_owned());
    }
    Ok(map)
}
