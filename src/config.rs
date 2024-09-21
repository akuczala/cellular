use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum SystemConfig {
    Conway,
    XYModel,
    Wave,
    Sandpile,
    Schrodinger,
}

#[derive(Deserialize)]
pub enum BoundaryConfig {
    Periodic,
    Constant,
    Free,
}

#[derive(Deserialize)]
pub struct Config {
    pub grid_width: u32,
    pub grid_height: u32,
    pub per_frame_updates: u32,
    pub system: SystemConfig,
    pub boundary: BoundaryConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            grid_width: 200,
            grid_height: 200,
            per_frame_updates: 1,
            system: SystemConfig::XYModel,
            boundary: BoundaryConfig::Periodic,
        }
    }
}

pub fn read_config() -> Config {
    fs::read_to_string("config.toml")
        .map_err(|e| e.to_string())
        .and_then(|contents| toml::from_str(&contents).map_err(|e| e.to_string()))
        .map_err(|msg| println!("{:}", msg))
        .unwrap_or_default()
}
