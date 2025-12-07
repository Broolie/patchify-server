use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use crate::schemas::AuthType;

//TODO: move somewhere else
impl<'de> Deserialize<'de> for AuthType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let valid = &["noauth", "password", "login", "token"];
        let value = String::deserialize(deserializer)?.to_lowercase();
        match value.as_str() {
            "noauth" => Ok(AuthType::NoAuth),
            "password" => Ok(AuthType::Password),
            "login" => Ok(AuthType::Login),
            "token" => Ok(AuthType::Token),
            _ => Err(serde::de::Error::unknown_variant(&value, valid)),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub available_auth_types: Vec<AuthType>, //TODO: implement bitflag deserialize, not vec jez
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
                Ok(config) => Ok(config),
                Err(err) => Err(err.to_string()),
            }
        } else {
            Err("unknown file format".to_owned())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_address: "0.0.0.0:5114".to_owned(),
            available_auth_types: Vec::new(),
        }
    }
}
