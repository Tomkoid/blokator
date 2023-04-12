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
    Actions, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, actions::{apply::apply_hosts, backup::backup, restore::restore_backup, add_repo::add_repo_action, del_repo::del_repo_action, list_repos::list_repos_action, add_repo_preset::add_repo_preset_action, del_repo_preset::del_repo_preset_action},
};

use crate::actions::sync::sync_repositories;

pub fn exec_command(args: &Args) {
    // Initialize colors and messages
    let colors = initialize_colors();
    let messages = Messages::new();

    match args.to_owned().command {
        Commands::Sync(_) => sync_repositories(args.to_owned()),
        Commands::Apply => apply_hosts(args.to_owned()),
        Commands::Backup => backup(),
        Commands::Restore => restore_backup(),
        Commands::AddRepo(a) => add_repo_action(a.repo, args.to_owned()),
        Commands::AddRepoPreset(a) => add_repo_preset_action(a.repo, args.to_owned()),
        Commands::DelRepo(a) => del_repo_action(a.repo),
        Commands::DelRepoPreset(a) => del_repo_preset_action(a.repo),
        Commands::ListRepos => list_repos_action(),
        _ => todo!()
    };

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
