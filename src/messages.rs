use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Messages {
    pub message: HashMap<String, String>,
    pub help_message: HashMap<String, String>,
    pub restore_message: HashMap<String, String>,
    pub backup_message: HashMap<String, String>,
    pub apply_message: HashMap<String, String>,
}

impl Messages {
    pub fn new() -> Self {
        toml::from_str(include_str!("messages/messages.toml")).unwrap()
    }
}

impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}
