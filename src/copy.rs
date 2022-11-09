// copy.rs
//
// Simple cross-platform and system-wide CLI adblocker
// Copyright (C) 2022 Tomáš Zierl
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

use std::io::ErrorKind;
use std::process::exit;

use crate::initialize_colors::initialize_colors;
use crate::{write::write_to_file, read::read_file_to_string, Actions};
use crate::messages::Messages;

pub fn copy(from: &str, to: &str, action: Actions) {
    let colors = initialize_colors();
    
    let messages: Messages = toml::from_str(include_str!("messages/messages.toml")).unwrap();

    let not_found_message = match action {
        Actions::Restore => {
            messages.restore_message.get("not_found").unwrap()
        },
        Actions::Backup => {
            messages.backup_message.get("not_found").unwrap()
        },
        Actions::Apply => {
            messages.apply_message.get("not_found").unwrap()
        }
    };
    
    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    not_found_message,
                    e,
                    e.kind()
                );
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.message.get("permission_denied").unwrap(),
                    e,
                    e.kind()
                );
                exit(1)
            }
            _ => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.message.get("unknown_error").unwrap(),
                    e,
                    e.kind()
                );
                exit(1)
            }
        }
    };

    write_to_file(to, output)
}
