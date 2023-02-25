#![allow(unreachable_code)]

use clap::Parser;
use dirs::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};

mod allowed_exit_functions;
mod android;
mod arguments;
pub mod colors;
mod copy;
mod handle_permissions;
mod initialize_colors;
mod initialize_dirs;
pub mod messages;
pub mod presets;
pub mod read;
mod repos;
mod services;
mod signal_handling;
mod sync;
mod commands;
pub mod write;
pub mod tor;
pub mod error;

#[cfg(target_family = "windows")]
mod windows;

use crate::android::list::list_devices;
use crate::android::restore::restore_android;
use crate::commands::exec_command;
use crate::initialize_colors::initialize_colors;
use crate::presets::preset::Presets;
use crate::services::networkmanager::restart_networkmanager;
#[cfg(target_family = "windows")]
use crate::windows::is_elevated;
use arguments::Args;

use crate::allowed_exit_functions::check_allowed_function;
use crate::android::apply::apply_android;
use crate::colors::Colors;
use crate::copy::copy;
use crate::handle_permissions::handle_permissions;
use crate::initialize_dirs::{already_initialized, initialize_dir};
use crate::messages::Messages;
use crate::read::read_file_to_string;
use crate::repos::{add_repo, del_repo, list_repos};
use crate::signal_handling::handle_signals;
use crate::sync::sync;
use crate::write::write_to_file;

#[cfg(target_family = "unix")]
const HOSTS_FILE: &str = "/etc/hosts";
#[cfg(target_family = "unix")]
const HOSTS_FILE_BACKUP_PATH: &str = "/etc/hosts.backup";

#[cfg(target_family = "windows")]
const HOSTS_FILE: &str = r"C:\Windows\System32\drivers\etc\hosts";
#[cfg(target_family = "windows")]
const HOSTS_FILE_BACKUP_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts.backup";

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

    // Initialize messages
    let messages: Messages = Messages::new();

    // Parse arguments
    let args = Args::parse();

    // Check if user is running blokator as root / administrator, otherwise exit
    handle_permissions();

    // Initialize important directories if they are not already initialized
    if !already_initialized() {
        initialize_dir();
    }

    exec_command(&args, state);

    // Check if allowed exit functions ended (else exit)
    check_allowed_function(&args);

    println!(
        "{}error:{} {}",
        colors.bold_red,
        colors.reset,
        messages.message.get("no_action_specified").unwrap()
    );
    println!(
        "{}HELP:{} {}",
        colors.bold_green,
        colors.reset,
        messages.help_message.get("no_action_specified").unwrap()
    );
    exit(1);
}

pub fn get_data_dir() -> String {
    format!("{}/.local/share/adblocker", home_dir().unwrap().display())
}
