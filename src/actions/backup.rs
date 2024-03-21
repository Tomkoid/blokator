use crate::{Actions, AppState, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH};

use super::*;

pub fn backup(app_state: &AppState) {
    let colors = &app_state.colors;
    let messages = &app_state.messages;

    copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
    println!(
        "  {}>{} {}",
        colors.bold_green,
        colors.reset,
        messages.message.get("created_backup").unwrap()
    );
    exit(0);
}
