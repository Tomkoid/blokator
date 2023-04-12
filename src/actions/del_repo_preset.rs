use crate::{presets::preset::Presets, repos::del_repo};

use super::*;

pub fn del_repo_preset_action(repo: String) {
    let repo = Presets::get(repo);
    del_repo(repo);
    exit(0);
}
