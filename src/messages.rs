use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub message: HashMap<String, String>,
    pub help_message: HashMap<String, String>,
    pub restore_message: HashMap<String, String>,
    pub backup_message: HashMap<String, String>,
    pub apply_message: HashMap<String, String>
}