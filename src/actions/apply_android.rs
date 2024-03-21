use crate::{android::apply::apply_android, AppState};

use super::*;

pub fn apply_android_action(app_state: &AppState, android_device: String) {
    let colors = Colors::new();
    let messages = Messages::new();

    apply_android(app_state, &android_device);
    println!(
        "[{}+{}] {}",
        colors.bold_green,
        colors.reset,
        messages
            .message
            .get("adblocker_started_no_networkmanager")
            .unwrap()
    );
    exit(0);
}
