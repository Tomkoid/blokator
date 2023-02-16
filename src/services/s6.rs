use std::process::Command;

pub fn networkmanager_s6_restart() -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("s6-svc")
        .args(["-r", "/run/service/NetworkManager-srv"])
        .status()
}
