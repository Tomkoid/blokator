use crate::repos::list_repos;

use super::*;

pub fn list_repos_action() {
    let repos_list = list_repos();

    for repo in repos_list {
        println!("{}", repo);
    }

    exit(0);
}
