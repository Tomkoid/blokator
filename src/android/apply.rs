// android/apply.rs
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

use std::process::Command;
use std::process::Stdio;
use std::process::exit;
use crate::Args;

use crate::get_data_dir;
use crate::initialize_colors::initialize_colors;

use super::checks::adb_exists;
use super::checks::device_ready;

pub fn apply_android(args: &Args) {
    let colors = initialize_colors();

    let android_device = match &args.android_device {
        Some(value) => value,
        None => {
            println!(
                "{}error:{} No device was specified\n{}HELP:{} try to specify device with `--android-device <device ID>`, list devices with `--list-devices` argument",
                colors.bold_red,
                colors.reset,
                colors.bold_green,
                colors.reset
            );
            exit(1);
        }
    };

    adb_exists();
    match device_ready(android_device) {
        true => {},
        false => {
            println!(
                "{}error:{} Device is not ready.",
                colors.bold_red,
                colors.reset
            );
            exit(1)
        }
    }

    // Mount / as read and write
    let mount_system_as_rw = Command::new("adb")
        .args(["-s", &android_device, "shell", "su", "-c", "mount", "-o", "rw,remount", "/"])
        .stdout(Stdio::piped())
        .status()
        .unwrap();

    if !mount_system_as_rw.success() {
        println!(
            "{}error:{} Failed to mount system as read & write",
            colors.bold_red,
            colors.reset
        );
        exit(1);
    }

    // Push temporary hosts file to /sdcard/hosts
    let push_sdcard = Command::new("adb")
        .stdout(Stdio::piped())
        .args(["-s", &android_device, "push", &(get_data_dir() + "/hosts"), "/sdcard/hosts"])
        .status()
        .unwrap();

    if !push_sdcard.success() {
        println!(
            "{}error:{} Cannot push the hosts file to the Android device",
            colors.bold_red,
            colors.reset
        );
        exit(1);
    }

    // Create a backup of current hosts file
    let copy_etc_hosts = Command::new("adb")
        .stdout(Stdio::piped())
        .args(["-s", &android_device, "shell", "su", "-c", "'cp", "/etc/hosts", "/etc/hosts.backup'"])
        .status()
        .unwrap();

    if !copy_etc_hosts.success() {
        println!(
            "{}error:{} Cannot make a backup of the hosts file",
            colors.bold_red,
            colors.reset
        );
        exit(1);
    }

    // Apply / Move hosts file
    let move_to_etc_hosts = Command::new("adb")
        .stdout(Stdio::piped())
        .args(["-s", &android_device, "shell", "su", "-c", "'mv", "/sdcard/hosts", "/etc/hosts'"])
        .status()
        .unwrap();

    if !move_to_etc_hosts.success() {
        println!(
            "{}error:{} Cannot apply the hosts file",
            colors.bold_red,
            colors.reset
        );
        exit(1);
    }

    // Mount / back as read only
    let mount_system_as_ro = Command::new("adb")
        .args(["-s", &android_device, "shell", "su", "-c", "mount", "-o", "ro,remount", "/"])
        .stdout(Stdio::piped())
        .status()
        .unwrap();

    if !mount_system_as_ro.success() {
        println!(
            "{}error:{} Failed to mount the system as read only",
            colors.bold_yellow,
            colors.reset
        );
    }
}
