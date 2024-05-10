use std::io::ErrorKind;
use std::process::exit;

use crate::logging::get_global_logger;
use crate::messages::Messages;
use crate::{read::read_file_to_string, write::write_to_file, Actions};

pub fn copy(from: &str, to: &str, action: Actions) {
    let logger = get_global_logger();

    let messages: Messages = Messages::new();

    let not_found_message = match action {
        Actions::Restore => messages.restore_message.get("not_found").unwrap(),
        Actions::Backup => messages.backup_message.get("not_found").unwrap(),
        Actions::Apply => messages.apply_message.get("not_found").unwrap(),
    };

    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                logger.log_error(not_found_message);
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                logger.log_error("permission_denied");
                exit(1)
            }
            _ => {
                logger.log_error("unknown_error");
                exit(1)
            }
        },
    };

    write_to_file(to, output)
}
