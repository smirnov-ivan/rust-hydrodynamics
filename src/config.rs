use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String
}

impl Config {
    
    pub fn load() -> Result<Self, String> {
        let cfg = fs::read_to_string("Settings.toml").expect("Error occured while reading config file");
        let config: Config = toml::from_str(&cfg).expect("Error occured while parsing config");
        Ok(config)
    }

}