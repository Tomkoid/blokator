use std::process::{exit, Command, Stdio};

use crate::{arguments::Args, initialize_colors::initialize_colors, messages::Messages};

use super::{
    checks::{adb_exists, device_ready},
    clear_line,
};

use spinners::Spinner;

pub fn restore_android(android_device: &String) {
    let colors = initialize_colors();
    let messages = Messages::new();

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

    let mut mount_system_as_rw_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_mounting_rw").unwrap().into(),
    );

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

    mount_system_as_rw_sp.stop();
    clear_line();

    let mut android_restore_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_restore").unwrap().into(),
    );

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

    android_restore_sp.stop();
    clear_line();

    let mut mount_system_as_ro_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_mounting_ro").unwrap().into(),
    );

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

    mount_system_as_ro_sp.stop();
    clear_line();

    println!("RESTORE COMPLETED");
}
