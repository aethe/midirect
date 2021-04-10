mod errors;
mod fs;
mod mapping;
mod midi;

use enigo::{Enigo, KeyboardControllable};
use errors::GenericError;
use fs::Config;
use mapping::Mapping;
use midi::MidiReader;
use std::{
    env,
    io::{stdin, Read},
};

fn main() -> Result<(), GenericError> {
    // get the file path of the config specified as the first argument
    let config_file_path = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .ok_or(GenericError::new(
            "no config file path specified".to_string(),
        ))?
        .clone();

    // read the config
    let config = fs::read_from_file(&config_file_path)?;
    let mapping = Mapping::from(&config)?;

    // connect to a midi port
    let _midi_reader = MidiReader::new(move |input| {
        // we expect at least 3 arguments in a midi message
        if input.len() < 3 {
            return;
        }

        // todo: find a way to share enigo between closure executions instead of creating a new instance every time
        let mut enigo = Enigo::new();

        // translate the message based on the first 4 bits of its first argument
        match input[0] & 0b11110000 {
            // note off
            0b10000000 => {
                if let Some(&key) = mapping.notes.get(&input[1]) {
                    enigo.key_up(key);
                }
            }
            // note on
            0b10010000 => {
                if let Some(&key) = mapping.notes.get(&input[1]) {
                    enigo.key_down(key);
                }
            }
            // control change
            0b10110000 => {
                if let Some(&key) = mapping.controls.get(&input[1]) {
                    match input[2] {
                        0 => enigo.key_up(key),
                        _ => enigo.key_down(key),
                    }
                }
            }
            _ => return,
        }
    })?;

    // run until the user presses return
    println!("running");
    println!("press return to exit");
    let _ = stdin().read_exact(&mut [0u8]).unwrap();

    Ok(())
}
