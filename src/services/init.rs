use std::path::Path;

use super::openrc::networkmanager_openrc_restart;
use super::runit::networkmanager_runit_restart;
use super::s6::networkmanager_s6_restart;
use super::systemd::networkmanager_systemd_restart;

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

#[derive(PartialEq)]
pub enum Init {
    SystemD,
    Runit,
    OpenRC,
    S6
}

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

pub fn exists_networkmanager() -> bool {
    return match get_init() {
        Some(_) => true,
        None => false
    }
}

pub fn restart_networkmanager_init() -> Result<std::process::ExitStatus, std::io::Error> {
    match get_init() {
        Some(Init::SystemD) => networkmanager_systemd_restart(),
        Some(Init::Runit) => networkmanager_runit_restart(),
        Some(Init::OpenRC) => networkmanager_openrc_restart(),
        Some(Init::S6) => networkmanager_s6_restart(),
        None => networkmanager_systemd_restart(),
    }
}
