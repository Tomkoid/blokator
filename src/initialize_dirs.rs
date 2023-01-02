// initialize_dirs.rs
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

use std::fs;
use std::path::Path;
use std::process::exit;

use crate::get_data_dir;
use crate::initialize_colors::initialize_colors;
use crate::write::write_to_file;

pub fn already_initialized() -> bool {
    let mut status: bool = true;
    if !Path::new(&(get_data_dir() + "/hosts")).exists() {
        status = false
    };
    if !Path::new(&(get_data_dir() + "/repos")).exists() {
        status = false
    };
    if !Path::new(&get_data_dir()).exists() {
        status = false
    };
    status
}

pub fn initialize_dir() {
    let colors = initialize_colors();

    fs::create_dir_all(get_data_dir()).unwrap_or_else(|e| {
        println!(
            "{}error:{} Error occurred when initializing dirs: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e,
            e.kind()
        );
        exit(1);
    });

    let stevenblack_hosts =
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string();
    let repos_file_location = format!("{}/repos", get_data_dir());

    // Create file for repos
    write_to_file(&repos_file_location, stevenblack_hosts);

    let local_hosts_location = format!("{}/hosts", get_data_dir());
    write_to_file(&local_hosts_location, "".to_string())
}
