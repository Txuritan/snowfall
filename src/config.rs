use std::fs::File;
use std::io::BufReader;

use ron::de::from_reader;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub jwt: String,
    #[serde(default = "database_default")]
    pub database: Database,
    #[serde(default = "server_default")]
    pub server: Server,
}

fn database_default() -> Database {
    Database::SQLite {
        file: "snowfall.db".to_string(),
    }
}

fn server_default() -> Server {
    Server {
        host: "127.0.0.1".to_string(),
        port: "50411".to_string(),
    }
}

#[derive(Clone, Deserialize)]
pub enum Database {
    SQLite { file: String },
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
}

#[derive(Clone, Deserialize)]
pub struct TLS {}

impl Config {
    pub fn load() -> Config {
        from_reader(BufReader::new(
            File::open("config.ron").expect("unable to open config file"),
        ))
        .expect("failed to load config")
    }
}
