use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    url: Option<String>,
    pool_size: Option<u32>,
}

pub fn read_toml() -> Config {
    let config_path = "config.toml";

    let mut config_file = match File::open(config_path) {
        Ok(f) => f,
        Err(e) => panic!("Error occurred opening file: {} - Err: {}", config_path, e),
    };

    let mut config = String::new();
    match config_file.read_to_string(&mut config) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {} - Err: {}", config_path, e),
    };
    toml::from_str::<Config>(&config).unwrap()
}

impl Config {
    pub fn db(&self) -> &Database {
        &self.database
    }
}

impl Database {
    pub fn url(&self) -> String {
        if let Some(s) = &self.url {
            s.clone()
        } else {
            String::default()
        }
    }
    pub fn pool_size(&self) -> u32 {
        if let Some(n) = self.pool_size {
            n
        } else {
            10
        }
    }
}
