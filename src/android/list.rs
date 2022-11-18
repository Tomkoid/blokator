// android/list.rs
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
use std::process::Command;
use std::process::Stdio;

use crate::initialize_colors::initialize_colors;

use super::checks::adb_exists;

pub fn list_devices() {
    let colors = initialize_colors();
    
    adb_exists();
    
    let devices = Command::new("adb")
        .stdout(Stdio::piped())
        .arg("devices")
        .output()
        .unwrap();

    let devices_output = String::from_utf8(devices.stdout).unwrap();

    let mut lines: i32 = 0;
    for _ in devices_output.lines() {
        lines += 1;
    }

    if lines == 2 {
        println!("No device found");
        exit(1);
    }

    println!("DEVICE ID");

    for (index, line) in devices_output.lines().enumerate() {
        if index == 0 {
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let mut device_id = "";
        let mut device_state = "";

        for (index, section) in line.split('\t').enumerate() {
            if index == 0 {
                device_id = section;
            }
            if index == 1 {
                device_state = section;
            }
        }

        let get_device_model = Command::new("adb")
            .args(["-s", device_id, "shell", "getprop", "ro.product.model"])
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let mut device_model = String::from_utf8(get_device_model.stdout).unwrap();

        if device_model.is_empty() {
            device_model = "unknown model".to_string();
        }
        
        println!(
            "{}{device_id}{} {}({}, in {device_state} state){}",
            colors.bold_white,
            colors.reset,
            colors.bold_gray,
            device_model.trim(),
            colors.reset
        );
    }
}
