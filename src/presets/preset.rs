use crate::actions::Colors;
use crate::messages::Messages;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Presets {
    pub preset: HashMap<String, String>,
}

impl Presets {
    pub fn new() -> Self {
        toml::from_str(include_str!("presets.toml")).unwrap()
    }

    pub fn get(query: String) -> String {
        let messages = Messages::new();
        let colors = Colors::new();
        let presets = Self::new();

        let preset_url = presets.preset.get(&query);

        if !presets.preset.contains_key(&query) {
            println!(
                "  {}>{} {}",
                colors.bold_red,
                colors.reset,
                messages.message.get("preset_notfound").unwrap()
            );
            exit(1)
        } else {
            preset_url.unwrap().to_string()
        }
    }
}

impl Default for Presets {
    fn default() -> Self {
        Self::new()
    }
}
