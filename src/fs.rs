use crate::errors::GenericError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

// ╔═╗╔═╗╔╗╔╔═╗╦╔═╗
// ║  ║ ║║║║╠╣ ║║ ╦
// ╚═╝╚═╝╝╚╝╚  ╩╚═╝

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub keys: HashMap<String, String>,
    pub controls: HashMap<String, String>,
}

impl Config {
    pub fn new(keys: HashMap<String, String>, controls: HashMap<String, String>) -> Self {
        Self { keys, controls }
    }
}

// ╦═╗╔═╗╔═╗╔╦╗╦╔╗╔╔═╗
// ╠╦╝║╣ ╠═╣ ║║║║║║║ ╦
// ╩╚═╚═╝╩ ╩═╩╝╩╝╚╝╚═╝

pub fn read_from_file(path: &str) -> Result<Config, GenericError> {
    let file = File::open(path)
        .map_err(|_| GenericError::new(format!("couldn't open the file at {}", path)))?;

    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader).map_err(|_| {
        GenericError::new(format!(
            "couldn't parse the contents of the file at {}",
            path
        ))
    })?;

    Ok(config)
}

// ╦ ╦╦═╗╦╔╦╗╦╔╗╔╔═╗
// ║║║╠╦╝║ ║ ║║║║║ ╦
// ╚╩╝╩╚═╩ ╩ ╩╝╚╝╚═╝

pub fn write_to_file(config: Config, path: &str) -> Result<(), GenericError> {
    let file = File::open(path)
        .or_else(|_| File::create(path))
        .map_err(|_| GenericError::new(format!("couldn't open the file at {}", path)))?;

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
        .map_err(|_| GenericError::new(format!("couldn't write to the file at {}", path)))?;

    Ok(())
}
