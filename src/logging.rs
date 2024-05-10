use colored::Colorize;

use crate::{actions::Messages, LOGGER};

#[derive(Debug, Clone)]
pub struct Logger {
    messages: Messages,
}

#[derive(Debug, PartialEq)]
pub enum LogType {
    Error,
    Help,
    Warning,
    Info,
    Success,
}

impl Logger {
    pub fn new(messages: &Messages) -> Self {
        Self {
            messages: messages.to_owned(),
        }
    }

    pub fn log_error(&self, error_type: &str) {
        self.log(LogType::Error, error_type);
    }

    pub fn log_help(&self, help_type: &str) {
        self.log(LogType::Help, help_type);
    }

    pub fn log_warning(&self, warning_type: &str) {
        self.log(LogType::Warning, warning_type);
    }

    pub fn log_info(&self, info_type: &str) {
        self.log(LogType::Info, info_type);
    }

    pub fn log_success(&self, success_msg: &str) {
        self.log(LogType::Success, success_msg)
    }

    fn log(&self, message_type: LogType, message: &str) {
        let start = match message_type {
            LogType::Error => "error:".bold().red(),
            LogType::Help => "help:".bold().blue(),
            LogType::Warning => "warning:".bold().yellow(),
            LogType::Info => "info:".bold().green(),
            LogType::Success => "success:".bold().green(),
        };

        let message_content = match message_type {
            LogType::Error => self.messages.message.get(message),
            LogType::Help => self.messages.help_message.get(message),
            LogType::Warning => self.messages.message.get(message),
            LogType::Info => self.messages.message.get(message),
            LogType::Success => self.messages.message.get(message),
        };

        let message = message.to_string();
        let message_content = message_content.unwrap_or(&message);

        if message_type == LogType::Error || message_type == LogType::Warning {
            eprintln!("{} {}", start, message_content)
        } else {
            println!("{} {}", start, message_content)
        }
    }
}

pub fn get_global_logger() -> Logger {
    LOGGER.with(|logger| logger.clone())
}
