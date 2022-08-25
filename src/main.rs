use clap::Parser;
use dirs::home_dir;
use std::process::exit;
use nix::unistd::Uid;
use std::path::Path;

pub mod read;
pub mod write;
pub mod messages;
pub mod colors;
mod copy;
mod sync;
mod systemd;
mod initialize_dirs;

use crate::colors::{Colors, check_no_color_env};
use crate::read::read_file_to_string;
use crate::systemd::networkmanager::{ 
                                    networkmanager_exists,
                                    networkmanager_restart
                                    };
use crate::initialize_dirs::{ already_initialized, initialize_dir };
use crate::sync::sync;
use crate::copy::copy;

const HOSTS_FILE: &str = "/etc/hosts";
const HOSTS_FILE_BACKUP_PATH: &str = "/etc/hosts.backup";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "Easy system-wide adblocker")]
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
    if !Uid::effective().is_root() {
        println!("{}==>{} Root is required to run the adblocker.", colors.bold_red, colors.reset);
        exit(1);
    }
   
    // Initialize important directories
    if !already_initialized() {
        initialize_dir();
    }

    if args.sync {
        if sync() {
            println!("{}==>{} Synced the adblocker.", colors.bold_green, colors.reset);
        } else {
            println!("{}==>{} No change.", colors.bold_yellow, colors.reset)
        }
        exit(0);
    }

    // Create backup to /etc/hosts.backup
    if args.backup {
        copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
        println!("{}==>{} Created backup.", colors.bold_green, colors.reset);
        exit(0);
    }

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        if read_file_to_string(HOSTS_FILE_BACKUP_PATH).unwrap() == read_file_to_string(HOSTS_FILE).unwrap() {
            println!("{}==>{} Backup already restored.", colors.bold_yellow, colors.reset);
            exit(1);
        }
        copy(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE, Actions::Restore);
        if networkmanager_exists() {
            let networkmanager_status = match networkmanager_restart() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            };

            if networkmanager_status.success() {
                println!("{}==>{} Restarted NetworkManager.service.", colors.bold_yellow, colors.reset);
            } else {
                println!("{}==>{} Cannot restart NetworkManager.service.", colors.bold_red, colors.reset)
            }
        } else {
            println!("{}==>{} Manually restart your networking service or restart the system to apply changes.", colors.bold_yellow, colors.reset);
        }
        println!("{}==>{} Restored the backup.", colors.bold_green, colors.reset);
        exit(0);
    }

    if args.apply {
        let local_hosts = format!(
            "{}/hosts",
            get_data_dir()
        );
        if !Path::new(&local_hosts).exists() {
            println!("{}==>{} Can't apply, because the local hosts are missing.", colors.bold_red, colors.reset);
            println!("{}Help:{} run blokator with `--sync` argument`", colors.bold_green, colors.reset);
            exit(1);
        } else if !Path::new(HOSTS_FILE).exists() {
            println!("{}==>{} Can't apply, because the /etc/hosts file is missing.", colors.bold_red, colors.reset);
            exit(1);
        }
        if read_file_to_string(HOSTS_FILE).unwrap() == read_file_to_string(&local_hosts).unwrap() {
            println!("{}==>{} Latest ad list update is already applied.", colors.bold_yellow, colors.reset);
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
                println!("{}==>{} Restarted NetworkManager.service.", colors.bold_green, colors.reset);
            } else {
                println!("{}==>{} Cannot restart NetworkManager.service.", colors.bold_red, colors.reset)
            }
        } else {
            println!("{}==>{} To apply the changes, manually restart your networking service or restart the system.", colors.bold_yellow, colors.reset);
        }

        println!("{}==>{} Started the adblocker.", colors.bold_green, colors.reset);
        exit(0);
    }

    println!("{}==>{} No action specified.", colors.bold_red, colors.reset);
    println!("{}Help:{} see all available arguments with `--help` argument", colors.bold_green, colors.reset);
    exit(1);
}

pub fn get_data_dir() -> String {
    format!(
        "{}/.local/share/adblocker",
        home_dir().unwrap().display()
    )
}

// fn get_home_dir() -> String {
//     format!("{}", home_dir().unwrap().display())
// }
