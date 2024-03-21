use crate::{android::restore::restore_android, AppState};

use super::*;

pub fn restore_android_action(app_state: &AppState, android_device: String) {
    restore_android(app_state, &android_device);

    exit(0);
}
