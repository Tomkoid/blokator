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
use crate::messages::CopyMessages;

pub fn copy(from: &str, to: &str, action: Actions) {
    let colors = initialize_colors();

    let messages = CopyMessages::new(action);

    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.not_found,
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
                    messages.permission_denied,
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
                    messages.unknown_error,
                    e,
                    e.kind()
                );
                exit(1)
            }
        }
    };

    write_to_file(to, output)
}
