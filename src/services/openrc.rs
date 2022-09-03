use std::process::Command;

pub fn networkmanager_openrc_restart() -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("rc-service")
        .args(["NetworkManager", "restart"])
        .status()
}
