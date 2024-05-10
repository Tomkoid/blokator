use crate::get_data_dir;
use crate::logging::get_global_logger;
use crate::read_file_to_string;
use crate::tor::if_onion_link;
use crate::write::write_to_file;
use crate::AppState;
use std::path::Path;
use std::process::exit;

async fn verify_repo(repo: &String, app_state: &AppState) {
    let args = &app_state.args;
    let logger = &app_state.logger;

    let mut client = reqwest::ClientBuilder::new();

    let tor_proxy = format!("socks5h://{}:{}", args.tor_bind_address, args.tor_port);

    if args.tor {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap())
    } else if if_onion_link(repo.to_string()) {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap());
    }

    client
        .build()
        .unwrap_or_else(|e| {
            logger.log_error(&format!("Failed to connect to the repo: {}", e));
            exit(1)
        })
        .get(repo)
        .send()
        .await
        .unwrap_or_else(|e| {
            logger.log_error(&format!("Failed to connect to the repo: {}", e));
            exit(1)
        });
}

pub fn list_repos() -> Vec<String> {
    let repos_file_location = format!("{}/repos", get_data_dir());

    let mut repos = "".to_string();
    if Path::new(&repos_file_location).exists() {
        repos = read_file_to_string(&repos_file_location).unwrap();
    }

    let mut repos_list: Vec<String> = [].to_vec();
    for repo in repos.lines() {
        if repo.is_empty() {
            continue;
        }

        repos_list.push(repo.to_string())
    }

    repos_list
}

pub async fn add_repo(repo: &String, app_state: &AppState) {
    let logger = &app_state.logger;

    let file_location = format!("{}/repos", get_data_dir());
    let mut output = read_file_to_string(&file_location).unwrap();

    for i in output.lines() {
        if i == repo {
            logger.log_error("The repo you're trying to add already exists in repos list");
            exit(1);
        }
    }

    output = format!("{}\n{}", output, repo);

    // Check if the repo responds
    verify_repo(repo, app_state).await;

    write_to_file(&file_location, output);

    logger.log_success("Added and verified the repo.");

    exit(0)
}

pub fn del_repo(repo: String) {
    let logger = get_global_logger();

    let repos_file_location = format!("{}/repos", get_data_dir());

    if Path::new(&repos_file_location).exists() {
        let mut repos = read_file_to_string(&repos_file_location).unwrap();
        if !repos.contains(&repo) {
            logger.log_error("The repo you're trying to delete doesn't exist");

            exit(1);
        }
        repos = repos.replace(&repo, "").replace("\n\n", "\n");
        write_to_file(&repos_file_location, repos);
        logger.log_success(&format!("Deleted {} from the repo list.", repo));
    } else {
        logger.log_error(&format!(
            "Failed to delete {} from the repo list, because the repo list doesn't exist.",
            repo
        ));
        exit(1);
    }
}
