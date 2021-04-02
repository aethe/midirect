use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct GenericError {
    pub message: String,
}

impl GenericError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.message.fmt(f)
    }
}

impl Error for GenericError {}
