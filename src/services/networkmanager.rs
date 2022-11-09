// services/networkmanger.rs
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

use crate::initialize_colors::initialize_colors;
use crate::services::init::restart_networkmanager_init;
use super::init::exists_networkmanager;
use crate::Messages;
use crate::get_init;

pub fn restart_networkmanager() {
    let colors = initialize_colors();
    let messages: Messages = toml::from_str(include_str!("../messages/messages.toml")).unwrap();

    if exists_networkmanager() {
        print!(
            "{}==>{} Restarting NetworkManager..",
            colors.bold_blue,
            colors.reset
        );

        let networkmanager_status = match restart_networkmanager_init() {
            Ok(s) => s,
                Err(e) => panic!("couldn't restart NetworkManager: {e}")
            };

        if networkmanager_status.success() {
            println!(" {}done{}", colors.bold_green, colors.reset);
        } else {
            // Init 2 = OpenRC
            /*
             * OpenRC sometime returns 1 as a exit code when printing errors and
             * warning, which is the same exit code
             */
            if get_init() == 2 {
                println!(" {}failed / warning{}", colors.bold_red, colors.reset);
            } else {
                println!(" {}failed{}", colors.bold_red, colors.reset);
            }
        }
    } else {
       println!("{}==>{} {}", colors.bold_yellow, colors.reset, messages.message.get("networkmanager_restart_manually").unwrap());
    }
}
