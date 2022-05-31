use crate::{client::Clients, types::Result};

/// Main application state
#[derive(Debug)]
pub struct State {
    /// Input file name
    filename: String,

    /// A map of clients with client ID as a key
    pub clients: Clients,
}

impl State {
    /// Returns new app's state
    pub fn new<T: AsRef<str>>(filename: T) -> Result<Self> {
        Ok(State {
            filename: filename.as_ref().to_string(),
            clients: Clients::new(),
        })
    }

    /// Returns file name
    pub fn filename(&self) -> &str {
        self.filename.as_str()
    }
}
