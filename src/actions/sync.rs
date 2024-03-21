use crate::AppState;

use super::*;

pub fn sync_repositories(app_state: &AppState) {
    let colors = &app_state.colors;
    let messages = &app_state.messages;
    let args = &app_state.args;

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

        // print!(
        //     "  [{}*{}] {} {}.. ",
        //     colors.bold_blue,
        //     colors.reset,
        //     messages.message.get("syncing").unwrap(),
        //     repo,
        // );

        let mut syncing_spinner = Spinner::new(
            SPINNER_TYPE,
            format!(
                "{}{}{} {}{}{}",
                colors.bold_blue,
                messages.message.get("syncing").unwrap(),
                colors.reset,
                colors.green,
                repo,
                colors.reset
            ),
        );

        std::io::stdout().flush().unwrap();

        let error = sync(repo, &args);

        if !error {
            syncing_spinner.stop_with_newline();
        } else {
            println!("{}error{}", colors.bold_red, colors.reset);
        }
    }

    let changed = local_hosts_output != read_file_to_string(&local_hosts).unwrap();

    #[cfg(target_os = "linux")]
    write_to_file(&hosts_temp, read_file_to_string(&local_hosts).unwrap());

    if changed {
        println!("{}", messages.message.get("synced_successfully").unwrap());

        #[cfg(target_os = "linux")]
        println!("{}", messages.message.get("wrote_temp_hosts").unwrap());
    } else {
        println!("{}", messages.message.get("nothing_changed").unwrap());
    }

    exit(0);
}
