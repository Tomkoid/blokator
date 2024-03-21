use crate::actions::Colors;
use crate::messages::Messages;

use std::process::exit;
use std::process::Command;
use std::process::Stdio;

use crate::get_data_dir;

use super::checks::adb_exists;
use super::checks::device_ready;
use super::clear_line;

use spinners::Spinner;

// Send notification and return bool with success state
pub fn send_notification(android_device: &String) -> bool {
    let result = Command::new("adb")
        .args([
            "-s",
            android_device,
            "shell",
            "cmd",
            "notification",
            "post",
            "-t",
            "'Blokator'",
            "'Blokator'",
            r"'Successfully applied'",
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .unwrap();

    result.success()
}

pub fn apply_android(android_device: &String) {
    let colors = Colors::new();
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
            "{}error:{} Failed to mount system as read & write",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    mount_system_as_rw_sp.stop();
    clear_line();

    let mut push_sdcard_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_temp_push").unwrap().into(),
    );

    // Push temporary hosts file to /sdcard/hosts
    let push_sdcard = Command::new("adb")
        .stdout(Stdio::piped())
        .args([
            "-s",
            android_device,
            "push",
            &(get_data_dir() + "/hosts"),
            "/sdcard/hosts",
        ])
        .status()
        .unwrap();

    if !push_sdcard.success() {
        println!(
            "{}error:{} Cannot push the hosts file to the Android device",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    push_sdcard_sp.stop();
    clear_line();

    let mut copy_etc_hosts_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages
            .message
            .get("android_backup_create")
            .unwrap()
            .into(),
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
            "/etc/hosts",
            "/etc/hosts.backup'",
        ])
        .status()
        .unwrap();

    if !copy_etc_hosts.success() {
        println!(
            "{}error:{} Cannot make a backup of the hosts file",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    copy_etc_hosts_sp.stop();
    clear_line();

    let mut move_to_etc_hosts_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_apply_hosts").unwrap().into(),
    );

    // Apply / Move hosts file
    let move_to_etc_hosts = Command::new("adb")
        .stdout(Stdio::piped())
        .args([
            "-s",
            android_device,
            "shell",
            "su",
            "-c",
            "'mv",
            "/sdcard/hosts",
            "/etc/hosts'",
        ])
        .status()
        .unwrap();

    if !move_to_etc_hosts.success() {
        println!(
            "{}error:{} Cannot apply the hosts file",
            colors.bold_red, colors.reset
        );
        exit(1);
    }

    move_to_etc_hosts_sp.stop();
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
            "{}error:{} Failed to mount the system as read only",
            colors.bold_yellow, colors.reset
        );
    }

    mount_system_as_ro_sp.stop();
    clear_line();

    let mut send_notification_sp = Spinner::new(
        spinners::Spinners::Dots2,
        messages.message.get("android_send_message").unwrap().into(),
    );

    // If send_notification was unsuccessful
    if !send_notification(android_device) {
        println!("{}error{}", colors.bold_yellow, colors.reset);
        exit(0);
    }

    send_notification_sp.stop();
    clear_line();
}
