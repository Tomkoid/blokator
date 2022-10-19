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

#![allow(unreachable_code)]

use clap::Parser;
use dirs::home_dir;
use std::fs;
use std::process::exit;
use std::path::Path;
use std::io::Write;
use std::sync::{ Arc, Mutex };

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
mod signal_handling;
mod allowed_exit_functions;
mod android;
mod arguments;

#[cfg(target_family = "windows")]
mod windows;

use arguments::Args;
use crate::android::checks::check_android_feature;
use crate::android::list::list_devices;
use crate::initialize_colors::initialize_colors;
use crate::services::networkmanager::restart_networkmanager;
#[cfg(target_family = "windows")]
use crate::windows::is_elevated;

use crate::colors::Colors;
use crate::messages::{GenericMessages, HelpMessages};
use crate::read::read_file_to_string;
use crate::initialize_dirs::{ already_initialized, initialize_dir };
use crate::sync::sync;
use crate::copy::copy;
use crate::repos::{add_repo, list_repos, del_repo};
use crate::handle_permissions::handle_permissions;
use crate::signal_handling::handle_signals;
use crate::allowed_exit_functions::check_allowed_function;
use crate::services::init::get_init;
use crate::android::apply::apply_android;

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

#[derive(PartialEq, Eq)]
pub enum Actions {
    Restore,
    Backup, 
    Apply,
}

fn main() {
    // This will be true if some action is running
    let state = Arc::new(Mutex::new(false));

    #[cfg(target_os = "linux")]
    let thread_state = Arc::clone(&state);

    #[cfg(target_os = "linux")]
    handle_signals(thread_state);

    // Initialize colors
    let colors = initialize_colors();

    // Parse arguments
    let args = Args::parse();

    // Check if user is running blokator as root / administrator, otherwise exit
    handle_permissions();

    // Initialize important directories if they are not already initialized
    if !already_initialized() {
        initialize_dir();
    }

    // List repos
    if args.list_repos {
        let repos_list = list_repos();
        for repo in repos_list {
            println!("{}", repo);
        }
        exit(0);
    }

    // Add repo
    if args.add_repo != "none" {
        *state.lock().unwrap() = true;
        add_repo(&args.add_repo, &args);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Delete repo
    if args.del_repo != "none" {
        *state.lock().unwrap() = true;
        del_repo(args.del_repo);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Sync all repositories
    if args.sync {
        *state.lock().unwrap() = true;
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

            print!(
                "  [{}*{}] Syncing {}.. ",
                colors.bold_blue,
                colors.reset,
                repo,
            );

            std::io::stdout().flush().unwrap();

            let time = std::time::SystemTime::now();

            sync(repo, &args);

            println!(
                "{}took {}ms{}",
                colors.bold_green,
                time.elapsed().expect("counting elapsed time").as_millis(),
                colors.reset
            )
        }

        let changed = local_hosts_output != read_file_to_string(&local_hosts).unwrap();

        if changed {
            println!(
                "  [{}+{}] Synced all repos successfully.",
                colors.bold_green,
                colors.reset
            );
        } else {
            println!(
                "  [{}-{}] Nothing changed.",
                colors.bold_yellow,
                colors.reset
            );
        }
        *state.lock().unwrap() = false;
    }

    // Create backup to /etc/hosts.backup
    if args.backup {
        *state.lock().unwrap() = true;
        copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
        println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.created_backup);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        *state.lock().unwrap() = true;
        if read_file_to_string(HOSTS_FILE_BACKUP_PATH).unwrap() == read_file_to_string(HOSTS_FILE).unwrap() {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.backup_already_restored);
            exit(1);
        }
        copy(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE, Actions::Restore);
        restart_networkmanager();
        println!("{}==>{} {}", colors.bold_green, colors.reset, MESSAGES.backup_restored);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Apply changes
    if args.apply {
        *state.lock().unwrap() = true;
        let local_hosts = format!(
            "{}/hosts",
            get_data_dir()
        );
        if !Path::new(&local_hosts).exists() {
            println!("  [{}*{}] {}", colors.bold_red, colors.reset, MESSAGES.local_hosts_missing);
            println!("  {}Help:{} {}", colors.bold_green, colors.reset, HELP_MESSAGES.local_hosts_missing);
            exit(1);
        } else if !Path::new(HOSTS_FILE).exists() {
            println!("  [{}*{}] {}", colors.bold_red, colors.reset, MESSAGES.etc_hosts_missing);
            exit(1);
        }
        if read_file_to_string(HOSTS_FILE).unwrap() == read_file_to_string(&local_hosts).unwrap() {
            println!("  [{}*{}] {}", colors.bold_yellow, colors.reset, MESSAGES.already_applied);
            exit(1);
        }
               
        if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
            // Backup /etc/hosts to /etc/hosts.backup
            copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup); 
        }

        // Rewrite /etc/hosts
        copy(&local_hosts, HOSTS_FILE, Actions::Apply);
        
        restart_networkmanager();

        println!("   {}>{} {}", colors.bold_green, colors.reset, MESSAGES.adblocker_started);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Apply changes on Android device (only if compiling with `feature` crate)
    if args.apply_android {
        *state.lock().unwrap() = true;

        check_android_feature();

        apply_android(&args);
        println!(
            "   {}>{} Started the adblocker, but you must reboot or restart your wifi adapter to see the changes",
            colors.bold_green,
            colors.reset
        );
        *state.lock().unwrap() = false;
        exit(0);
    }

    if args.list_devices {
        check_android_feature();

        *state.lock().unwrap() = true;
        list_devices();
        *state.lock().unwrap() = false;
        
        exit(0);
    }

    // Check if allowed exit functions ended (else exit)
    check_allowed_function(&args);

    println!("{}error:{} {}", colors.bold_red, colors.reset, MESSAGES.no_action_specified);
    println!("{}HELP:{} {}", colors.bold_green, colors.reset, HELP_MESSAGES.no_action_specified);
    exit(1);
}

pub fn get_data_dir() -> String {
    format!(
        "{}/.local/share/adblocker",
        home_dir().unwrap().display()
    )
}
