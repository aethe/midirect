mod errors;
mod fs;
mod mapping;
mod midi;

use enigo::{Enigo, KeyboardControllable};
use errors::GenericError;
use fs::Config;
use mapping::Mapping;
use midi::MidiReader;
use std::io::{stdin, Read};

fn main() -> Result<(), GenericError> {
    let config = fs::read_from_file("rsc/config.json")?;
    let mapping = Mapping::from(&config)?;

    let _midi_reader = MidiReader::new(move |input| {
        if input.len() < 3 {
            return;
        }

        // todo: find a way to share enigo between closure executions instead of creating a new instance every time
        let mut enigo = Enigo::new();

        match input[0] & 0b11110000 {
            // note off
            0b10000000 => {
                if let Some(&key) = mapping.keys.get(&input[1]) {
                    enigo.key_up(key);
                }
            }
            // note on
            0b10010000 => {
                if let Some(&key) = mapping.keys.get(&input[1]) {
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

    println!("running");
    println!("press any key to exit");

    // wait until some key is pressed
    let _ = stdin().read_exact(&mut [0u8]).unwrap();

    Ok(())
}
