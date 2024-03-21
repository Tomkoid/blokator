use crate::actions::Colors;
use std::io::ErrorKind;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

pub fn device_ready(device: &str) -> bool {
    let devices = Command::new("adb")
        .stdout(Stdio::piped())
        .arg("devices")
        .output()
        .unwrap();

    let devices_output = String::from_utf8(devices.stdout).unwrap();

    for line in devices_output.lines() {
        let mut line_splitted = "";

        for (index, line) in line.trim().split('\t').enumerate() {
            if index == 0 {
                line_splitted = line;
                break;
            }
        }

        // If line contains device ID
        if line_splitted == device && line.trim().contains("device") {
            return true;
        }
    }

    false
}

pub fn adb_exists() {
    let colors = Colors::new();

    match Command::new("adb").stdout(Stdio::piped()).spawn() {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!(
                    "{}error:{} ADB command doesn't exist",
                    colors.bold_red, colors.reset
                );
                exit(1);
            } else {
                println!(
                    "{}error:{} Some strange error occurred",
                    colors.bold_red, colors.reset
                );
                exit(1);
            }
        }
    }
}
