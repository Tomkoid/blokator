// colors.rs
//
// Simple cross-platform and system-wide CLI adblocker
// Copyright (C) 2022 Tomáš Zierl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::env;

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
    pub fn new() -> Colors {
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

    pub fn new_without_colors() -> Colors {
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
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}

pub fn check_no_color_env() -> bool {
    let no_color_env = env::var_os("NO_COLOR");

    if no_color_env.is_none() {
        return false;
    }

    env::var_os("NO_COLOR").unwrap() == "1" || env::var_os("NO_COLOR").unwrap() == "true"
}
