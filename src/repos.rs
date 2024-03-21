use crate::actions::Colors;
use crate::get_data_dir;
use crate::read_file_to_string;
use crate::tor::if_onion_link;
use crate::write::write_to_file;
use crate::AppState;
use std::path::Path;
use std::process::exit;

fn verify_repo(repo: &String, app_state: &AppState) {
    let colors = &app_state.colors;
    let args = &app_state.args;

    let mut client = reqwest::blocking::ClientBuilder::new();

    let tor_proxy = format!("socks5h://{}:{}", args.tor_bind_address, args.tor_port);

    if args.tor {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap())
    } else if if_onion_link(repo.to_string()) {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap());
    }

    client
        .build()
        .unwrap()
        .get(repo)
        .send()
        .unwrap_or_else(|e| {
            println!(
                "{}error:{} Failed to connect to the repo: {}",
                colors.bold_red, colors.reset, e,
            );
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

pub fn add_repo(repo: &String, app_state: &AppState) {
    let colors = &app_state.colors;

    let file_location = format!("{}/repos", get_data_dir());
    let mut output = read_file_to_string(&file_location).unwrap();

    for i in output.lines() {
        if i == repo {
            println!(
                "{}error:{} The repo you're trying to add already exists in repos list.",
                colors.bold_red, colors.reset
            );
            exit(1);
        }
    }

    output = format!("{}\n{}", output, repo);

    // Check if the repo responds
    verify_repo(repo, &app_state);

    write_to_file(&file_location, output);

    println!(
        "{}success:{} Added and verified the repo.",
        colors.bold_green, colors.reset
    );
}

pub fn del_repo(repo: String) {
    let colors = Colors::new();

    let repos_file_location = format!("{}/repos", get_data_dir());

    if Path::new(&repos_file_location).exists() {
        let mut repos = read_file_to_string(&repos_file_location).unwrap();
        if !repos.contains(&repo) {
            println!(
                "{}error:{} The repo you're trying to delete doesn't exist",
                colors.bold_red, colors.reset
            );
            exit(1);
        }
        repos = repos.replace(&repo, "").replace("\n\n", "\n");
        write_to_file(&repos_file_location, repos);
        println!(
            "{}success:{} Deleted {} from the repo list.",
            colors.bold_green, colors.reset, repo
        );
    } else {
        println!(
            "{}error:{} Failed to delete {} from the repo list, because the repo list doesn't exist.",
            colors.bold_red,
            colors.reset,
            repo
        );
        exit(1);
    }
}
