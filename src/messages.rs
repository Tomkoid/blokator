use crate::Actions;

pub struct CopyMessages<'a> {
    pub not_found: &'a str,
    pub permission_denied: &'a str,
    pub unknown_error: &'a str,
}

impl CopyMessages<'_> {
    pub fn new(action: Actions) -> CopyMessages<'static> {
        match action {
            Actions::Restore => { CopyMessages {
                not_found: "Tried to restore the backup, but it doesn't exist",
                permission_denied: "Permission Denied",
                unknown_error: "Error occurred"
            } },
            Actions::Backup => { CopyMessages {
                not_found: "Tried to backup the /etc/hosts file, but it doesn't exist",
                permission_denied: "Permission denied",
                unknown_error: "Error occured"
            } },
            Actions::Apply => { CopyMessages {
                not_found: "Tried to start the adblocker, but the /etc/hosts file doesn't exist",
                permission_denied: "Permission Denied",
                unknown_error: "Error occured"
            } }
        }
    }
}
