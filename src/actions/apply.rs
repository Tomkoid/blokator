use crate::{
    services::networkmanager::restart_networkmanager, Actions, AppState, HOSTS_FILE,
    HOSTS_FILE_BACKUP_PATH,
};

use super::*;

pub fn apply_hosts(app_state: &AppState) {
    let colors = &app_state.colors;
    let messages = &app_state.messages;

    let local_hosts = format!("{}/hosts", get_data_dir());
    if !Path::new(&local_hosts).exists() {
        eprintln!(
            "  [{}*{}] {}",
            colors.bold_red,
            colors.reset,
            messages.message.get("local_hosts_missing").unwrap()
        );
        eprintln!(
            "  {}HELP:{} {}",
            colors.bold_green,
            colors.reset,
            messages.help_message.get("local_hosts_missing").unwrap()
        );
        exit(1);
    } else if !Path::new(HOSTS_FILE).exists() {
        eprintln!(
            "  [{}*{}] {}",
            colors.bold_red,
            colors.reset,
            messages.message.get("etc_hosts_missing").unwrap()
        );
        exit(1);
    }
    if read_file_to_string(HOSTS_FILE).unwrap() == read_file_to_string(&local_hosts).unwrap() {
        eprintln!(
            "  [{}*{}] {}",
            colors.bold_yellow,
            colors.reset,
            messages.message.get("already_applied").unwrap()
        );
        exit(1);
    }

    if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
        // Backup /etc/hosts to /etc/hosts.backup
        copy(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH, Actions::Backup);
    }

    // Rewrite /etc/hosts
    copy(&local_hosts, HOSTS_FILE, Actions::Apply);

    restart_networkmanager();

    println!(
        "  {}>{} {}",
        colors.bold_green,
        colors.reset,
        messages.message.get("adblocker_started").unwrap()
    );
    exit(0);
}
