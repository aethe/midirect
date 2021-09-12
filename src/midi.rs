use crate::GenericError;
use midir::{MidiInput, MidiInputConnection};

pub struct MidiReader {
    connection: MidiInputConnection<()>,
}

impl MidiReader {
    pub fn new<F>(on_input: F) -> Result<Self, GenericError>
    where
        F: Fn(&[u8]) + Send + 'static,
    {
        let input = MidiInput::new("midirect-input")
            .map_err(|_| GenericError::new("could not start midi input".to_string()))?;

        let ports = input.ports();
        let port = ports
            .first()
            .ok_or(GenericError::new("no available midi ports".to_string()))?;

        let connection = input
            .connect(
                port,
                "midirect-input",
                move |_, message, _| {
                    on_input(message);
                },
                (),
            )
            .map_err(|_| GenericError::new("could not connect to a midi port".to_string()))?;

        Ok(Self { connection })
    }
}
