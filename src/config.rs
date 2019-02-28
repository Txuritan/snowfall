use std::fs::File;
use std::io::BufReader;

use ron::de::from_reader;
use serde::Deserialize;

// TODO: figure out how to merge
// const DEFAULTS: &str = include_str!("../config.ron");

#[derive(Clone, Deserialize)]
pub struct Config {
    pub jwt: String,
    pub database: Database,
    pub server: Server,
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
    pub fn new() -> Config {
        Default::default()
    }
}

impl Default for Config {
    fn default() -> Config {
        from_reader(BufReader::new(
            File::open("config.ron").expect("unable to open config file"),
        ))
        .expect("failed to load config")
    }
}
