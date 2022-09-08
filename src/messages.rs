// messages.rs
//
// Simple cross-platform and system-wide CLI adblocker
// Copyright (C) 2022 Tomáš Zierl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::Actions;

pub struct GenericMessages<'a> {
    pub root_is_required: &'a str,
    pub synced: &'a str,
    pub synced_no_change: &'a str,
    pub created_backup: &'a str,
    pub backup_already_restored: &'a str,
    pub backup_restored: &'a str,
    pub networkmanager_restart: &'a str,
    pub networkmanager_couldnt_restart: &'a str,
    pub networkmanager_restart_manually: &'a str,
    pub local_hosts_missing: &'a str,
    pub etc_hosts_missing: &'a str,
    pub already_applied: &'a str,
    pub adblocker_started: &'a str,
    pub no_action_specified: &'a str
}

pub struct HelpMessages<'a> {
    pub local_hosts_missing: &'a str,
    pub no_action_specified: &'a str
}

pub struct CopyMessages<'a> {
    pub not_found: &'a str,
    pub permission_denied: &'a str,
    pub unknown_error: &'a str,
}

impl GenericMessages<'_> {
    pub const fn new() -> GenericMessages<'static> {
        GenericMessages { 
            root_is_required: "You need to run this command with root privilegies, to run the adblocker",
            synced: "Synced the adblocker.",
            synced_no_change: "No change",
            created_backup: "Backup created.",
            backup_already_restored: "Backup already restored.",
            backup_restored: "Backup restored.",
            networkmanager_restart: "Restarted NetworkManager.service successfully.",
            networkmanager_couldnt_restart: "Couldn't restart NetworkManager.service.",
            networkmanager_restart_manually: "Manually restart the networking service or restart the system.",
            local_hosts_missing: "Can't apply, because the local hosts are missing.",
            etc_hosts_missing: "Can't apply, because the /etc/hosts file is missing.",
            already_applied: "Latest ad list already applied.",
            adblocker_started: "Blokator started.",
            no_action_specified: "No action specified."
        }
    }
}

impl HelpMessages<'_> {
    pub const fn new() -> HelpMessages<'static> {
        HelpMessages { 
            local_hosts_missing: "run blokator with `--sync` argument.",
            no_action_specified: "run blokator with `--help` argument to see all available commands."
        }
    }
}

impl CopyMessages<'_> {
    pub fn new(action: Actions) -> CopyMessages<'static> {
        match action {
            Actions::Restore => { CopyMessages {
                not_found: "Tried to restore the backup, but it doesn't exist",
                permission_denied: "Permission Denied",
                unknown_error: "Error occurred"
            } },
            Actions::Backup => { CopyMessages {
                not_found: "Tried to backup the /etc/hosts file, but it doesn't exist",
                permission_denied: "Permission denied",
                unknown_error: "Error occured"
            } },
            Actions::Apply => { CopyMessages {
                not_found: "Tried to start the adblocker, but the /etc/hosts file doesn't exist",
                permission_denied: "Permission Denied",
                unknown_error: "Error occured"
            } }
        }
    }
}
