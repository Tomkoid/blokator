use crate::{presets::preset::Presets, repos::add_repo, AppState};

use super::*;

pub async fn add_repo_preset_action(repo: String, app_state: &AppState) {
    let repo = Presets::get(repo);
    add_repo(&repo, app_state).await;
    exit(0);
}
