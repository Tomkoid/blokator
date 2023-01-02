// presets/preset.rs
//
// Simple cross-platform and system-wide CLI adblocker
// Copyright (C) 2023 Tomáš Zierl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::initialize_colors;
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
        let colors = initialize_colors();
        let presets = Self::new();

        let preset_url = presets.preset.get(&query);

        if presets.preset.get(&query).is_none() {
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
