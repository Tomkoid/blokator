use std::{
    fs::{self},
    io::Write,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

use crate::{
    android::{apply::apply_android, list::list_devices, restore::restore_android},
    arguments::{Args, Commands},
    copy::copy,
    get_data_dir,
    initialize_colors::initialize_colors,
    messages::Messages,
    presets::preset::Presets,
    read::read_file_to_string,
    repos::{add_repo, del_repo, list_repos},
    services::networkmanager::restart_networkmanager,
    sync::sync,
    write::write_to_file,
    Actions, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, actions::{apply::apply_hosts, backup::backup},
};

use crate::actions::sync::sync_repositories;

pub fn exec_command(args: &Args) {
    // Initialize colors and messages
    let colors = initialize_colors();
    let messages = Messages::new();

    // List repos
    if args.list_repos {
        let repos_list = list_repos();
        for repo in repos_list {
            println!("{}", repo);
        }
        exit(0);
    }

    // Add repo
    if args.add_repo.is_some() {
        add_repo(&args.add_repo.clone().unwrap(), &args);
        exit(0);
    }

    // Add repo from preset
    if args.add_repo_preset.is_some() {
        let repo = Presets::get(args.add_repo_preset.clone().unwrap());
        add_repo(&repo, &args);
        exit(0);
    }

    // Delete repo
    if args.del_repo != "none" {
        del_repo(args.clone().del_repo);
        exit(0);
    }

    // Delete repo from preset
    if args.del_repo_preset.is_some() {
        let repo = Presets::get(args.clone().del_repo_preset.unwrap());
        del_repo(repo);
        exit(0);
    }

    match args.to_owned().command {
        Commands::Sync(_) => sync_repositories(args.to_owned()),
        Commands::Apply => apply_hosts(args.to_owned()),
        Commands::Backup => backup(),
        _ => todo!()
    };

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
            println!(
                "  {}>{} {}",
                colors.bold_red,
                colors.reset,
                messages.restore_message.get("not_found").unwrap()
            );
            exit(1);
        }
        if read_file_to_string(HOSTS_FILE_BACKUP_PATH).unwrap()
            == read_file_to_string(HOSTS_FILE).unwrap()
        {
            println!(
                "  {}>{} {}",
                colors.bold_yellow,
                colors.reset,
                messages.message.get("backup_already_restored").unwrap()
            );
            exit(1);
        }
        copy(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE, Actions::Restore);
        restart_networkmanager();
        println!(
            "  {}>{} {}",
            colors.bold_green,
            colors.reset,
            messages.message.get("backup_restored").unwrap()
        );
        exit(0);
    }

    // Apply changes on Android device (only if compiling with `feature` crate)
    if args.apply_android {

        apply_android(&args);
        println!(
            "[{}+{}] {}",
            colors.bold_green,
            colors.reset,
            messages
                .message
                .get("adblocker_started_no_networkmanager")
                .unwrap()
        );
        exit(0);
    }

    if args.restore_android {
        restore_android(&args);

        exit(0);
    }

    if args.list_devices {
        list_devices();

        exit(0);
    }
}
