use std::process::Command;

pub fn networkmanager_runit_restart() -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("sv")
        .args(["restart", "NetworkManager"])
        .status()
}
