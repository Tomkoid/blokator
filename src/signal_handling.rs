// signal_handling.rs
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

use std::{thread, sync::Arc, sync::Mutex};
use std::process::exit;
use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};

use crate::initialize_colors::initialize_colors;

// Signal handling (ex: CTRL + c)
pub fn handle_signals(thread_state: Arc<Mutex<bool>>) {
    let mut signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();

    thread::spawn(move || {
        let mut already_pressed = false;
        for _ in signals.forever() {
            if *thread_state.lock().unwrap() {
                if !already_pressed {
                    println!(
                        " {}Force kill with CTRL + C{}",
                        initialize_colors().bold_red,
                        initialize_colors().reset
                    );
                    already_pressed = true;
                    continue;
                } else {
                    exit(2);
                }
            }
            println!(
                " {}Exiting..{}",
                initialize_colors().bold_red,
                initialize_colors().reset
            );
            exit(1);
        }
    });

}
