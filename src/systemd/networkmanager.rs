use std::{path::Path, process::Command};

const NETWORKMANAGER_SERVICE_PATH: &str = "/etc/systemd/system/multi-user.target.wants/NetworkManager.service";

pub fn networkmanager_exists() -> bool {
    Path::new(NETWORKMANAGER_SERVICE_PATH).exists()
}

pub fn networkmanager_restart() -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("systemctl")
        .args(["restart", "NetworkManager"])
        .status()
}
