use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub start_url: String,
    pub search_url: String,
    pub icon: Option<String>,
    pub frameless: bool,
    pub transparent: bool,
    pub always_on_top: bool,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string("config.toml")?;
        Ok(toml::from_str(&data)?)
    }
}