use crate::repos::add_repo;

use super::*;

pub fn add_repo_action(repo: String, args: Args) {
    add_repo(&repo, &args);
    exit(0);
}
