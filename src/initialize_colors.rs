// initialize_colors.rs
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

use crate::Colors;

#[cfg(target_family = "unix")]
use crate::colors::check_no_color_env;

pub fn initialize_colors() -> Colors {
    #[cfg(target_family = "windows")]
    return Colors::new_without_colors();

    #[cfg(target_family = "unix")]
    {
        let mut colors = Colors::new_without_colors();

        // If user runs blokator with NO_COLOR flag
        #[cfg(target_family = "unix")]
        if !check_no_color_env() {
            colors = Colors::new();
        }

        colors
    }
}
