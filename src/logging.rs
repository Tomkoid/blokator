use crate::actions::{Colors, Messages};

#[derive(Debug, Clone)]
pub struct Logger {
    colors: Colors,
    messages: Messages,
}

impl Logger {
    pub fn new(colors: &Colors, messages: &Messages) -> Self {
        Self {
            colors: colors.clone(),
            messages: messages.clone(),
        }
    }

    pub fn log_error(&self, error_type: &str) {
        println!(
            "{}error:{} {}",
            self.colors.bold_red,
            self.colors.reset,
            self.messages.message.get(error_type).unwrap()
        );
    }

    pub fn log_help(&self, help_type: &str) {
        println!(
            "{}help:{} {}",
            self.colors.bold_blue,
            self.colors.reset,
            self.messages.help_message.get(help_type).unwrap()
        );
    }

    pub fn log_warning(&self, warning_type: &str) {
        println!(
            "{}warning:{} {}",
            self.colors.bold_yellow,
            self.colors.reset,
            self.messages.message.get(warning_type).unwrap()
        );
    }

    pub fn log_info(&self, info_type: &str) {
        println!(
            "{}info:{} {}",
            self.colors.bold_green,
            self.colors.reset,
            self.messages.message.get(info_type).unwrap()
        );
    }
}
