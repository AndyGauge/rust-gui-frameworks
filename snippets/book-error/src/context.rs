//! Adding context as an error travels up the call stack.

use crate::{Error, Result};

/// Add a human message to an error, or turn a `None` into an error.
pub trait Context<T> {
    fn context(self, message: impl Into<String>) -> Result<T>;
}

impl<T, E: Into<Error>> Context<T> for std::result::Result<T, E> {
    fn context(self, message: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::Context { context: message.into(), source: Box::new(e.into()) })
    }
}

impl<T> Context<T> for Option<T> {
    fn context(self, message: impl Into<String>) -> Result<T> {
        self.ok_or_else(|| Error::Missing(message.into()))
    }
}
