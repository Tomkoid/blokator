use crate::{android::list::list_devices, AppState};

use super::*;

pub fn list_devices_action(app_state: &AppState) {
    list_devices(app_state);

    exit(0);
}
