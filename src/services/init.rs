use std::path::Path;
use std::process::Command;

const NETWORKMANAGER_SYSTEMD_SERVICE_PATH: &str =
    "/etc/systemd/system/multi-user.target.wants/NetworkManager.service";
const NETWORKMANAGER_RUNIT_SERVICE_PATH: &str = "/etc/runit/runsvdir/current/NetworkManager/run";
const NETWORKMANAGER_OPENRC_SERVICE_PATH: &str = "/etc/runlevels/default/NetworkManager";
const NETWORKMANAGER_S6_SERVICE_PATH: &str = "/etc/s6/adminsv/default/contents.d/NetworkManager";

/*
 * 0 - systemd
 * 1 - runit
 * 2 - openrc
 * 3 - s6
*/

pub struct NetworkManager;

#[derive(PartialEq)]
pub enum Init {
    SystemD,
    Runit,
    OpenRC,
    S6,
}

impl NetworkManager {
    pub fn exists() -> bool {
        Init::get_init().is_some()
    }

    pub fn restart() -> Result<std::process::ExitStatus, std::io::Error> {
        match Init::get_init() {
            Some(Init::SystemD) => Init::systemd_restart(),
            Some(Init::Runit) => Init::runit_restart(),
            Some(Init::OpenRC) => Init::openrc_restart(),
            Some(Init::S6) => Init::s6_restart(),
            None => Init::systemd_restart(),
        }
    }
}

impl Init {
    pub fn get_init() -> Option<Init> {
        if Path::new(NETWORKMANAGER_SYSTEMD_SERVICE_PATH).exists() {
            return Some(Init::SystemD);
        } else if Path::new(NETWORKMANAGER_RUNIT_SERVICE_PATH).exists() {
            return Some(Init::Runit);
        } else if Path::new(NETWORKMANAGER_OPENRC_SERVICE_PATH).exists() {
            return Some(Init::OpenRC);
        } else if Path::new(NETWORKMANAGER_S6_SERVICE_PATH).exists() {
            return Some(Init::S6);
        }

        None
    }

    pub fn systemd_restart() -> Result<std::process::ExitStatus, std::io::Error> {
        Command::new("systemctl")
            .args(["restart", "NetworkManager"])
            .status()
    }

    pub fn runit_restart() -> Result<std::process::ExitStatus, std::io::Error> {
        Command::new("sv")
            .args(["restart", "NetworkManager"])
            .status()
    }

    pub fn openrc_restart() -> Result<std::process::ExitStatus, std::io::Error> {
        Command::new("rc-service")
            .args(["NetworkManager", "restart"])
            .status()
    }

    pub fn s6_restart() -> Result<std::process::ExitStatus, std::io::Error> {
        Command::new("s6-svc")
            .args(["-r", "/run/service/NetworkManager-srv"])
            .status()
    }
}
