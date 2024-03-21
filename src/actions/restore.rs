use crate::{
    services::networkmanager::restart_networkmanager, Actions, AppState, HOSTS_FILE,
    HOSTS_FILE_BACKUP_PATH,
};

use super::*;

pub fn restore_backup(app_state: &AppState) {
    let colors = &app_state.colors;
    let messages = &app_state.messages;

    if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
        eprintln!(
            "  {}>{} {}",
            colors.bold_red,
            colors.reset,
            messages.restore_message.get("not_found").unwrap()
        );
        exit(1);
    }
    if read_file_to_string(HOSTS_FILE_BACKUP_PATH).unwrap()
        == read_file_to_string(HOSTS_FILE).unwrap()
    {
        eprintln!(
            "  {}>{} {}",
            colors.bold_yellow,
            colors.reset,
            messages.message.get("backup_already_restored").unwrap()
        );
        exit(1);
    }
    copy(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE, Actions::Restore);
    restart_networkmanager();
    println!(
        "  {}>{} {}",
        colors.bold_green,
        colors.reset,
        messages.message.get("backup_restored").unwrap()
    );
    exit(0);
}
