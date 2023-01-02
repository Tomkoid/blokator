// error.rs
//
// Simple cross-platform and system-wide CLI adblocker
// Copyright (C) 2023 Tomáš Zierl
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

pub fn check_http_error(data: &String) -> bool {
    let mut error: bool = false;
    if data.contains("<title>404 Not Found</title>") {
        error = true;
    }
    error
}
