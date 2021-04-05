use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
};

pub struct GenericError {
    pub message: String,
}

impl GenericError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Debug for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.message)
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.message)
    }
}

impl Error for GenericError {}
