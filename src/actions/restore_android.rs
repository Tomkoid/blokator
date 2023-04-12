use crate::android::restore::restore_android;

use super::*;

pub fn restore_android_action(android_device: String) {
    restore_android(&android_device);

    exit(0);
}
