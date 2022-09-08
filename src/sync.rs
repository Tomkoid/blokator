// sync.rs
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

use std::{process::exit, path::Path};

use crate::{write::write_to_file, get_data_dir, read::read_file_to_string, initialize_colors::initialize_colors};

pub fn sync(repo: &str) {
    let colors = initialize_colors(); 

    let response = ureq::get(repo).call();
    
    let local_hosts = format!(
        "{}/hosts",
        get_data_dir()
    );

    let response = match response {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ureq::ErrorKind::Dns => {
                println!(
                    "==> Connection failed. (Check your internet connection): {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            },
            _ => {
                println!(
                    "Error occurred: {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            }
        }
    };

    let resp = response.into_string().unwrap_or_else(|e| {
        println!(
            "{}==>{} Error when converting response to String: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e,
            e.kind()
        );
        exit(1)
    });

    if Path::new(&local_hosts).exists() {
        write_to_file(&local_hosts, read_file_to_string(&local_hosts).unwrap() + &resp + "\n\n");
    } else {
        write_to_file(&local_hosts, resp);
    }
}
