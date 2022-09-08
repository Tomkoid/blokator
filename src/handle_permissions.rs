// handle_permissions.rs
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

use std::process::exit;
use crate::initialize_colors;

use crate::GenericMessages;

#[cfg(target_family = "unix")]
use nix::unistd::Uid;

#[cfg(target_family = "windows")]
use crate::is_elevated;

const MESSAGES: GenericMessages = GenericMessages::new();

pub fn handle_permissions() {
    let colors = initialize_colors();

    // Check if the program is running with root permissions
    #[cfg(target_family = "unix")]
    if !Uid::effective().is_root() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required);
        exit(1);
    }

    #[cfg(target_family = "windows")]
    if !is_elevated() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required);
        exit(1);
    }
}
