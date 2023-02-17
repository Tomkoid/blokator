use std::process::{Command, exit, Stdio};

use crate::{initialize_colors::initialize_colors, arguments::Args};

use super::{apply::{print_msg, print_done}, checks::{adb_exists, device_ready}};

pub fn restore_android(args: &Args) {
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
        true => {}
        false => {
            println!(
                "{}error:{} Device is not ready.",
                colors.bold_red, colors.reset
            );
            exit(1)
        }
    }

    print_msg("android_mounting_rw");

    // Mount / as read and write
    let mount_system_as_rw = Command::new("adb")
        .args([
            "-s",
            android_device,
            "shell",
            "su",
            "-c",
            "mount",
            "-o",
            "rw,remount",
            "/",
        ])
        .stdout(Stdio::null())
        .status()
        .unwrap();

    if !mount_system_as_rw.success() {
        println!(
            "  {}error:{} Failed to mount system as read & write",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    print_done();

    
    print_msg("android_restore");
    
    // Create a backup of current hosts file
    let copy_etc_hosts = Command::new("adb")
        .stdout(Stdio::piped())
        .args([
            "-s",
            android_device,
            "shell",
            "su",
            "-c",
            "'cp",
            "/etc/hosts.backup",
            "/etc/hosts'",
        ])
        .status()
        .unwrap();

    if !copy_etc_hosts.success() {
        println!(
            "  {}error:{} Cannot restore a backup of the hosts file",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    print_done();

    print_msg("android_mounting_ro");

    // Mount / back as read only
    let mount_system_as_ro = Command::new("adb")
        .args([
            "-s",
            android_device,
            "shell",
            "su",
            "-c",
            "mount",
            "-o",
            "ro,remount",
            "/",
        ])
        .stdout(Stdio::piped())
        .status()
        .unwrap();

    if !mount_system_as_ro.success() {
        println!(
            "  {}error:{} Failed to mount the system as read only",
            colors.bold_yellow, colors.reset
        );
    }

    print_done();
}
