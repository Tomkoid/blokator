use crate::{repos::add_repo, AppState};

use super::*;

pub fn add_repo_action(repo: String, app_state: &AppState) {
    add_repo(&repo, &app_state);
    exit(0);
}
