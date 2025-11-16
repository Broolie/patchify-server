use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bind_address: String,
}

impl Config {
    pub fn new(config_path: &Path) -> Result<Self, String> {
        if let Some(ext) = config_path.extension() {
            if ext != "json" {
                return Err("Currently only json format is supported for config files".to_owned());
            }

            let file_result = File::open(config_path);
            if file_result.is_err() {
                return Err(file_result.err().unwrap().to_string());
            }

            let file = file_result.ok().unwrap();
            let reader = BufReader::new(file);
            let mut deserializer = serde_json::Deserializer::from_reader(reader);
            match Config::deserialize(&mut deserializer) {
                Ok(config) => return Ok(config),
                Err(err) => return Err(err.to_string())
            }
        }
        else {
            Err("unknown file format".to_owned())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bind_address: "localhost:5114".to_owned() }
    }
}
