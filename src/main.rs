use clap::Parser;
use dirs::home_dir;
use std::process::exit;
use std::path::Path;

#[cfg(target_os = "linux")]
use nix::unistd::Uid;

pub mod read;
pub mod write;
pub mod messages;
pub mod colors;
mod copy;
mod sync;
mod systemd;
mod initialize_dirs;

#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "windows")]
use crate::windows::is_elevated;

use crate::colors::{Colors, check_no_color_env};
use crate::messages::{GenericMessages, HelpMessages};
use crate::read::read_file_to_string;
use crate::systemd::networkmanager::{ 
                                    networkmanager_exists,
                                    networkmanager_restart
                                    };
use crate::initialize_dirs::{ already_initialized, initialize_dir };
use crate::sync::sync;
use crate::copy::copy;

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
    backup: bool
}

#[derive(PartialEq, Eq)]
pub enum Actions {
    Restore,
    Backup, 
    Apply,
}

fn main() {
    let colors: Colors;

    // If user runs blokator with NO_COLOR flag
    if check_no_color_env() {
        colors = Colors::new_without_colors();
    } else {
        colors = Colors::new();
    }

    let args = Args::parse();

    // Check if the program is running with root permissions
    #[cfg(target_family = "unix")]
    if !Uid::effective().is_root() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required,);
        exit(1);
    }

    #[cfg(target_family = "windows")]
    if !is_elevated() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required);
        exit(1);
    }
   
    // Initialize important directories
    if !already_initialized() {
        initialize_dir();
    }

    if args.sync {
        if sync() {
            println!("{}==>{} {}", MESSAGES.synced, colors.bold_green, colors.reset);
        } else {
            println!("{}==>{} {}", colors.bold_yellow, colors.reset, MESSAGES.synced_no_change)
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
        if networkmanager_exists() {
            let networkmanager_status = match networkmanager_restart() {
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
        
        if networkmanager_exists() {
            let networkmanager_status = match networkmanager_restart() {
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
