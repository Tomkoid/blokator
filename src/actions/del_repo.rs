use crate::repos::del_repo;

use super::*;

pub fn del_repo_action(repo: String) {
    del_repo(repo);
    exit(0);
}
