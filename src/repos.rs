use std::fmt::format;
use std::process::exit;
use std::path::Path;
use crate::Colors;
use crate::check_no_color_env;
use crate::get_data_dir;
use crate::read_file_to_string;
use crate::write::write_to_file;

fn verify_repo(repo: String) {
    let mut colors = Colors::new_without_colors();

    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

    ureq::get(&repo).call().unwrap_or_else(|e| {
        println!(
            "{}==>{} Failed to connect to the repo: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e, 
            e.kind()
        );
        exit(1)
    });
}

pub fn list_repos() -> Vec<String> {
    let repos_file_location = format!(
        "{}/repos",
        get_data_dir()
    );

    let mut repos = "".to_string();
    if Path::new(&repos_file_location).exists() {
        repos = read_file_to_string(&repos_file_location).unwrap();
    }

    let mut repos_list: Vec<String> = [].to_vec();
    for repo in repos.lines() {
        if repo == "" {
            continue;
        }

        repos_list.push(repo.to_string())
    }

    repos_list
}

pub fn add_repo(repo: String) {
    let mut colors = Colors::new_without_colors();

    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

    let file_location = format!("{}/repos", get_data_dir());
    let mut output = read_file_to_string(&file_location).unwrap();

    for i in output.lines() {
        println!("{}:{}", i, repo);
        if i == repo {
            println!(
                "{}==>{} The repo you're trying to add, already exists in repos list.",
                colors.bold_red,
                colors.reset
            );
            exit(1);
        }
    }

    output = format!("{}\n{}", output, repo);

    // Check if the repo responds
    verify_repo(repo);

    write_to_file(&file_location, output);

    println!(
        "{}==>{} Added and verified the repo.",
        colors.bold_green,
        colors.reset
    );
}
