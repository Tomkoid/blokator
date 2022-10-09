// android/checks.rs
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
use std::io::ErrorKind;
use std::process::Stdio;
use crate::initialize_colors;

pub fn check_android_feature() {
    #[allow(unused_variables)]
    let colors = initialize_colors();

    #[cfg(not(feature = "android"))]
    {
        println!(
            "   {}>{} To use this feature, you need to compile Blokator with `android` feature.",
            colors.bold_red,
            colors.reset
        );
        exit(1);
    }

}

pub fn device_ready(device: &str) -> bool {
    let devices = Command::new("adb")
        .stdout(Stdio::piped())
        .arg("devices")
        .output()
        .unwrap();

    let devices_output = String::from_utf8(devices.stdout).unwrap();
    
    for line in devices_output.lines() {
        let mut line_splitted = "";

        for (index, line) in line.trim().split("\t").enumerate() {
            if index == 0 { 
                line_splitted = line;
                break;
            }
        }

        // If line contains device ID
        if line_splitted == device && line.trim().contains("device") { return true; }
    }

    false
}

pub fn adb_exists() {
    let colors = initialize_colors();

    match Command::new("adb").stdout(Stdio::piped()).spawn() {
        Ok(_) => {},
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!(
                    "{}error:{} ADB command doesn't exist",
                    colors.bold_red,
                    colors.reset
                );
                exit(1);
            } else {
                println!(
                    "{}error:{} Some strange error occurred",
                    colors.bold_red,
                    colors.reset
                );
                exit(1);
            }
        },
    }
}
