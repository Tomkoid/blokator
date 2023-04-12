use std::{path::Path, io::Write, fs};
use std::process::exit;

use crate::{get_data_dir, read::read_file_to_string, colors::Colors, messages::Messages, sync::sync, write::write_to_file, arguments::Args};

pub fn sync_repositories(args: Args) {
    let colors = Colors::new();
    let messages = Messages::new();

    // Sync all repositories
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
        std::process::exit(1);
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

    exit(0);
}
