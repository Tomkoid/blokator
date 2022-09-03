use std::path::Path;

use super::openrc::networkmanager_openrc_restart;
use super::systemd::networkmanager_systemd_restart;
use super::runit::networkmanager_runit_restart;

const NETWORKMANAGER_SYSTEMD_SERVICE_PATH: &str = "/etc/systemd/system/multi-user.target.wants/NetworkManager.service";
const NETWORKMANAGER_RUNIT_SERVICE_PATH: &str = "/etc/runit/runsvdir/current/NetworkManager/run";
const NETWORKMANAGER_OPENRC_SERVICE_PATH: &str = "/etc/runlevels/default/NetworkManager";

/*
 * 0 - systemd
 * 1 - runit
 * 2 - openrc
*/
fn get_init() -> i32 {
    if Path::new(NETWORKMANAGER_SYSTEMD_SERVICE_PATH).exists() {
        return 0;
    } else if Path::new(NETWORKMANAGER_RUNIT_SERVICE_PATH).exists() {
        return 1;
    } else if Path::new(NETWORKMANAGER_OPENRC_SERVICE_PATH).exists() {
        return 2;
    }

    -1
}

pub fn exists_networkmanager() -> bool {
    get_init() != -1
}

pub fn restart_networkmanager() -> Result<std::process::ExitStatus, std::io::Error> {
    match get_init() {
        0 => networkmanager_systemd_restart(),
        1 => networkmanager_runit_restart(),
        2 => networkmanager_openrc_restart(),
        _ => networkmanager_systemd_restart()
    }
}
