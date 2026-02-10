use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub start_url: String,
    pub search_url: String,
    pub icon: Option<String>,
    pub frameless: bool,
    pub transparent: bool,
    pub always_on_top: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_url: String::new(),
            search_url: String::new(),
            icon: Some("assets/icon.png".to_string()),
            frameless: false,
            transparent: false,
            always_on_top: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // Look in:
        // Windows: %AppData%/webslab/config.toml
        // Linux: ~/.config/webslab/config.toml
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("webslab");

        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            // Create the directory and a default config if it doesn't exist
            std::fs::create_dir_all(&config_dir)?;
            let default_config = toml::to_string(&Config::default())?;
            std::fs::write(&config_path, default_config)?;
        }

        let data = std::fs::read_to_string(config_path)?;
        Ok(toml::from_str(&data)?)
    }
}