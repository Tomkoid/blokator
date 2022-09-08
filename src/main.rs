// main.rs
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

use clap::Parser;
use dirs::home_dir;
use std::fs;
use std::process::exit;
use std::path::Path;

pub mod read;
pub mod write;
pub mod messages;
pub mod colors;
mod copy;
mod sync;
mod services;
mod initialize_dirs;
mod initialize_colors;
mod repos;
mod handle_permissions;

#[cfg(target_family = "windows")]
mod windows;

use crate::initialize_colors::initialize_colors;
#[cfg(target_family = "windows")]
use crate::windows::is_elevated;

use crate::colors::{Colors, check_no_color_env};
use crate::messages::{GenericMessages, HelpMessages};
use crate::read::read_file_to_string;
use crate::services::init::{ restart_networkmanager, exists_networkmanager };
use crate::initialize_dirs::{ already_initialized, initialize_dir };
use crate::sync::sync;
use crate::copy::copy;
use crate::repos::{add_repo, list_repos, del_repo};
use crate::handle_permissions::handle_permissions;

#[cfg(target_family = "unix")]
const HOSTS_FILE: &str = "/etc/hosts";
#[cfg(target_family = "unix")]
const HOSTS_FILE_BACKUP_PATH: &str = "/etc/hosts.backup";

#[cfg(target_family = "windows")]
const HOSTS_FILE: &str = r"C:\Windows\System32\drivers\etc\hosts";
#[cfg(target_family = "windows")]
const HOSTS_FILE_BACKUP_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts.backup";

const MESSAGES: GenericMessages = GenericMessages::new();
const HELP_MESSAGES: HelpMessages = HelpMessages::new();

#[derive(Parser, Debug)]
#[clap(author = "Tomas Zierl", version, about, long_about = "Easy system-wide adblocker")]
struct Args {
    /// Start the adblocker
    #[clap(short, long, value_parser, default_value_t = false)]
    apply: bool,

    /// Sync the adblocker
    #[clap(short, long, value_parser, default_value_t = false)]
    sync: bool,

    /// Restore /etc/hosts backup
    #[clap(short, long, value_parser, default_value_t = false)]
    restore: bool,

    /// Create a backup to /etc/hosts.backup
    #[clap(short, long, value_parser, default_value_t = false)]
    backup: bool,

    /// Add repo for hosts files
    #[clap(short = 'm', long, value_parser, default_value = "none")]
    add_repo: String,

    /// List all repos
    #[clap(short, long, value_parser, default_value_t = false)]
    list_repos: bool,

    /// Delete specified repo from the repo list
    #[clap(short, long, value_parser, default_value = "none")]
    del_repo: String
}

#[derive(PartialEq, Eq)]
pub enum Actions {
    Restore,
    Backup, 
    Apply,
}

fn main() {
    let colors = initialize_colors();

    let args = Args::parse();

    // Check if user is running blokator as root / administrator
    handle_permissions();

    // Initialize important directories
    if !already_initialized() {
        initialize_dir();
    }

    if args.list_repos {
        let repos_list = list_repos();
        for repo in repos_list {
            println!("{}", repo);
        }
        exit(0);
    }

    if args.add_repo != "none" {
        add_repo(args.add_repo);
        exit(0);
    }

    if args.del_repo != "none" {
        del_repo(args.del_repo);
        exit(0);
    }

    if args.sync {
        let repos_file_location = format!(
            "{}/repos",
            get_data_dir()
        );

        let local_hosts = format!(
            "{}/hosts",
            get_data_dir()
        );

        let local_hosts_output = read_file_to_string(&local_hosts).unwrap();
        
        if Path::new(&local_hosts).exists() {
            fs::write(&local_hosts, "").unwrap();
        }

        let repos = read_file_to_string(&repos_file_location).unwrap();
        for repo in repos.lines() {
            if repo.is_empty() {
                continue;
            }

            println!(
                "{}==>{} Syncing {}..",
                colors.bold_blue,
                colors.reset,
                repo,
            );

            sync(repo)
        }

        let changed = local_hosts_output != read_file_to_string(&local_hosts).unwrap();

        if changed {
            println!(
                "{}==>{} Synced all repos successfully.",
                colors.bold_green,
                colors.reset
            );
        } else {
            println!(
                "{}==>{} Nothing changed.",
                colors.bold_yellow,
                colors.reset
            );
        }

        exit(0);
    }

    // Create backup to /etc/hosts.backup
    if args.backup {
        copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
        println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.created_backup);
        exit(0);
    }

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        if read_file_to_string(HOSTS_FILE_BACKUP_PATH).unwrap() == read_file_to_string(HOSTS_FILE).unwrap() {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.backup_already_restored);
            exit(1);
        }
        copy(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE, Actions::Restore);
        if exists_networkmanager() {
            let networkmanager_status = match restart_networkmanager() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            };

            if networkmanager_status.success() {
                println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.networkmanager_restart);
            } else {
                println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.networkmanager_couldnt_restart)
            }
        } else {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.networkmanager_restart_manually);
        }
        println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.backup_restored);
        exit(0);
    }

    if args.apply {
        let local_hosts = format!(
            "{}/hosts",
            get_data_dir()
        );
        if !Path::new(&local_hosts).exists() {
            println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.local_hosts_missing);
            println!("{}Help:{} {}", colors.bold_green, colors.reset, HELP_MESSAGES.local_hosts_missing);
            exit(1);
        } else if !Path::new(HOSTS_FILE).exists() {
            println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.etc_hosts_missing);
            exit(1);
        }
        if read_file_to_string(HOSTS_FILE).unwrap() == read_file_to_string(&local_hosts).unwrap() {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.already_applied);
            exit(1);
        }
               
        if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
            // Backup /etc/hosts to /etc/hosts.backup
            copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup); 
        }

        // Rewrite /etc/hosts
        copy(&local_hosts, HOSTS_FILE, Actions::Apply);
        
        if exists_networkmanager() {
            let networkmanager_status = match restart_networkmanager() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            };

            if networkmanager_status.success() {
                println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.networkmanager_restart);
            } else {
                println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.networkmanager_couldnt_restart);
            }
        } else {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.networkmanager_restart_manually);
        }

        println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.adblocker_started);
        exit(0);
    }

    println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.no_action_specified);
    println!("{}Help:{} {}", colors.bold_green, colors.reset, HELP_MESSAGES.no_action_specified);
    exit(1);
}

pub fn get_data_dir() -> String {
    format!(
        "{}/.local/share/adblocker",
        home_dir().unwrap().display()
    )
}
