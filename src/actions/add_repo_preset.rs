use crate::{presets::preset::Presets, repos::add_repo};

use super::*;

pub fn add_repo_preset_action(repo: String, args: Args) {
    let repo = Presets::get(repo);
    add_repo(&repo, &args);
    exit(0);
}
