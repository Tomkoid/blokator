use std::{
    fs::{self},
    io::Write,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

use crate::{
    android::{apply::apply_android, list::list_devices, restore::restore_android},
    arguments::Args,
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
    Actions, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH,
};

pub fn exec_command(args: &Args, state: Arc<Mutex<bool>>) {
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
        *state.lock().unwrap() = true;
        add_repo(&args.add_repo.clone().unwrap(), &args);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Add repo from preset
    if args.add_repo_preset.is_some() {
        *state.lock().unwrap() = true;
        let repo = Presets::get(args.add_repo_preset.clone().unwrap());
        add_repo(&repo, &args);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Delete repo
    if args.del_repo != "none" {
        *state.lock().unwrap() = true;
        del_repo(args.clone().del_repo);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Delete repo from preset
    if args.del_repo_preset.is_some() {
        *state.lock().unwrap() = true;
        let repo = Presets::get(args.clone().del_repo_preset.unwrap());
        del_repo(repo);
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Sync all repositories
    if args.sync {
        *state.lock().unwrap() = true;
        let repos_file_location = format!("{}/repos", get_data_dir());
        let hosts_temp = "/tmp/blokator".to_string();

        let local_hosts = format!("{}/hosts", get_data_dir());

        let local_hosts_output = read_file_to_string(&local_hosts).unwrap();

        if Path::new(&local_hosts).exists() {
            fs::write(&local_hosts, "").unwrap();
        }

        let repos = read_file_to_string(&repos_file_location).unwrap();
        if repos.trim().is_empty() {
            println!(
                "  [{}*{}] {}",
                colors.bold_blue,
                colors.reset,
                messages.message.get("no_repos_to_sync").unwrap()
            );
            exit(1);
        }
        for repo in repos.lines() {
            if repo.is_empty() {
                continue;
            }

            print!(
                "  [{}*{}] {} {}.. ",
                colors.bold_blue,
                colors.reset,
                messages.message.get("syncing").unwrap(),
                repo,
            );

            std::io::stdout().flush().unwrap();

            let time = std::time::SystemTime::now();

            let error = sync(repo, &args);

            if !error {
                println!(
                    "{}took {}ms{}",
                    colors.bold_green,
                    time.elapsed().expect("counting elapsed time").as_millis(),
                    colors.reset
                )
            } else {
                println!("{}error{}", colors.bold_red, colors.reset);
            }
        }

        let changed = local_hosts_output != read_file_to_string(&local_hosts).unwrap();

        #[cfg(target_os = "linux")]
        write_to_file(&hosts_temp, read_file_to_string(&local_hosts).unwrap());

        if changed {
            println!(
                "  [{}+{}] {}",
                colors.bold_green,
                colors.reset,
                messages.message.get("synced_successfully").unwrap()
            );

            #[cfg(target_os = "linux")]
            println!(
                "  [{}>{}] {}",
                colors.bold_green,
                colors.reset,
                messages.message.get("wrote_temp_hosts").unwrap()
            );
        } else {
            println!(
                "  [{}-{}] {}",
                colors.bold_yellow,
                colors.reset,
                messages.message.get("nothing_changed").unwrap()
            );
        }
        *state.lock().unwrap() = false;
    }

    // Create backup to /etc/hosts.backup
    if args.backup {
        *state.lock().unwrap() = true;
        copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
        println!(
            "  {}>{} {}",
            colors.bold_green,
            colors.reset,
            messages.message.get("created_backup").unwrap()
        );
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        *state.lock().unwrap() = true;
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
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Apply changes
    if args.apply {
        *state.lock().unwrap() = true;
        let local_hosts = format!("{}/hosts", get_data_dir());
        if !Path::new(&local_hosts).exists() {
            println!(
                "  [{}*{}] {}",
                colors.bold_red,
                colors.reset,
                messages.message.get("local_hosts_missing").unwrap()
            );
            println!(
                "  {}HELP:{} {}",
                colors.bold_green,
                colors.reset,
                messages.help_message.get("local_hosts_missing").unwrap()
            );
            exit(1);
        } else if !Path::new(HOSTS_FILE).exists() {
            println!(
                "  [{}*{}] {}",
                colors.bold_red,
                colors.reset,
                messages.message.get("etc_hosts_missing").unwrap()
            );
            exit(1);
        }
        if read_file_to_string(HOSTS_FILE).unwrap() == read_file_to_string(&local_hosts).unwrap() {
            println!(
                "  [{}*{}] {}",
                colors.bold_yellow,
                colors.reset,
                messages.message.get("already_applied").unwrap()
            );
            exit(1);
        }

        if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
            // Backup /etc/hosts to /etc/hosts.backup
            copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
        }

        // Rewrite /etc/hosts
        copy(&local_hosts, HOSTS_FILE, Actions::Apply);

        restart_networkmanager();

        println!(
            "  {}>{} {}",
            colors.bold_green,
            colors.reset,
            messages.message.get("adblocker_started").unwrap()
        );
        *state.lock().unwrap() = false;
        exit(0);
    }

    // Apply changes on Android device (only if compiling with `feature` crate)
    if args.apply_android {
        *state.lock().unwrap() = true;

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
        *state.lock().unwrap() = false;
        exit(0);
    }

    if args.restore_android {
        *state.lock().unwrap() = true;
        restore_android(&args);
        *state.lock().unwrap() = false;

        exit(0);
    }

    if args.list_devices {
        *state.lock().unwrap() = true;
        list_devices();
        *state.lock().unwrap() = false;

        exit(0);
    }
}
