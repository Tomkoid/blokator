use std::env;

#[derive(Debug, Clone)]
pub struct Colors {
    pub bold_white: String,
    pub bold_gray: String,
    pub bold_red: String,
    pub bold_green: String,
    pub bold_yellow: String,
    pub bold_blue: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub reset: String,
}

impl Colors {
    pub fn new() -> Self {
        #[cfg(target_family = "windows")]
        return Colors::new_without_colors();

        #[cfg(target_family = "unix")]
        {
            let mut colors = Self::get_colors_without_colors();

            // If user runs blokator with NO_COLOR flag
            #[cfg(target_family = "unix")]
            if !Self::check_no_color_env() {
                colors = Self::get_colors();
            }

            colors
        }
    }

    fn get_colors() -> Self {
        Colors {
            bold_white: "\x1b[1;40m".to_string(),
            bold_gray: "\x1b[1;90m".to_string(),
            bold_red: "\x1b[1;31m".to_string(),
            bold_green: "\x1b[1;32m".to_string(),
            bold_yellow: "\x1b[1;33m".to_string(),
            bold_blue: "\x1b[1;34m".to_string(),
            red: "\x1b[31m".to_string(),
            green: "\x1b[32m".to_string(),
            yellow: "\x1b[33m".to_string(),
            reset: "\x1b[0m".to_string(),
        }
    }

    // great name lol
    fn get_colors_without_colors() -> Self {
        Colors {
            bold_white: "".to_string(),
            bold_gray: "".to_string(),
            bold_red: "".to_string(),
            bold_green: "".to_string(),
            bold_yellow: "".to_string(),
            bold_blue: "".to_string(),
            red: "".to_string(),
            green: "".to_string(),
            yellow: "".to_string(),
            reset: "".to_string(),
        }
    }

    pub fn check_no_color_env() -> bool {
        let no_color_env = env::var_os("NO_COLOR");

        if no_color_env.is_none() {
            return false;
        }

        env::var_os("NO_COLOR").unwrap() == "1" || env::var_os("NO_COLOR").unwrap() == "true"
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}
