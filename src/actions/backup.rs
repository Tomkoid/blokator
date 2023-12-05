use crate::{Actions, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH};

use super::*;

pub fn backup() {
    let colors = Colors::new();
    let messages = Messages::new();

    copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
    println!(
        "  {}>{} {}",
        colors.bold_green,
        colors.reset,
        messages.message.get("created_backup").unwrap()
    );
    exit(0);
}
