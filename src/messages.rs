// messages.rs
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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
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
