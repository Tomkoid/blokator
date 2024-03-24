use crate::AppState;

use super::*;

pub async fn sync_repositories(app_state: &AppState) {
    let colors = &app_state.colors;
    let messages = &app_state.messages;

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

    println!(
        "{}{}{} {}{}",
        colors.bold_blue,
        messages.message.get("syncing").unwrap(),
        colors.reset,
        colors.green,
        colors.reset
    );

    let mut thread_joins: Vec<tokio::task::JoinHandle<String>> = Vec::new();

    for repo in repos.lines() {
        if repo.is_empty() {
            continue;
        }

        let repo = repo.to_string();

        let app_state_cloned = app_state.clone();

        let thread_handle = tokio::task::spawn_blocking(move || sync_repo(repo, &app_state_cloned));

        thread_joins.push(thread_handle);
    }

    for thread_join in thread_joins {
        let thread_join = thread_join.await;

        if thread_join.is_err() {
            eprintln!("  {}> error{}", colors.bold_red, colors.reset);
        }

        let result = thread_join.unwrap();

        println!("  {}>{} {}", colors.bold_green, colors.reset, result);
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

fn sync_repo(repo: String, app_state: &AppState) -> String {
    // print!(
    //     "  [{}*{}] {} {}.. ",
    //     colors.bold_blue,
    //     colors.reset,
    //     messages.message.get("syncing").unwrap(),
    //     repo,
    // );

    std::io::stdout().flush().unwrap();

    let error = sync(&repo, &app_state.args);

    if error {
        eprintln!(
            "  {}> error{}",
            app_state.colors.bold_red, app_state.colors.reset
        );
    }

    repo
}
