use std::process::Command;

pub fn networkmanager_systemd_restart() -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("systemctl")
        .args(["restart", "NetworkManager"])
        .status()
}
