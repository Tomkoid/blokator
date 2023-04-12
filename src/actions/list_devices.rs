use crate::android::list::list_devices;

use super::*;

pub fn list_devices_action() {
    list_devices();

    exit(0);
}
