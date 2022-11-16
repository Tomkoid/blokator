// arguments.rs
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

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(
    author = "Tomáš Zierl",
    version,
    about,
    long_about = "Easy system-wide adblocker"
)]
pub struct Args {
    /// Start the adblocker
    #[clap(short, long, value_parser, default_value_t = false)]
    pub apply: bool,

    /// Start adblocker on your Android phone with ADB (experimental, root required)
    #[clap(long, value_parser, default_value_t = false)]
    pub apply_android: bool,

    /// Specify android device (with device ID) (list devices with `--list-devices`)
    #[clap(long, value_parser)]
    pub android_device: Option<String>,

    /// List all Android devices (need to have USB debugging on)
    #[clap(long, value_parser, default_value_t = false)]
    pub list_devices: bool,

    /// Sync the adblocker
    #[clap(short, long, value_parser, default_value_t = false)]
    pub sync: bool,

    /// Restore /etc/hosts backup
    #[clap(short, long, value_parser, default_value_t = false)]
    pub restore: bool,

    /// Create a backup to /etc/hosts.backup
    #[clap(short, long, value_parser, default_value_t = false)]
    pub backup: bool,

    /// Add repo for hosts files
    #[clap(short = 'm', long, value_parser)]
    pub add_repo: Option<String>,

    /// List all repos
    #[clap(short, long, value_parser, default_value_t = false)]
    pub list_repos: bool,

    /// Delete specified repo from the repo list
    #[clap(short, long, value_parser, default_value = "none")]
    pub del_repo: String,

    /// Add repo from preset
    #[clap(short = 'M', long, value_parser)]
    pub add_repo_preset: Option<String>,

    /// Delete repo from preset
    #[clap(short = 'D', long, value_parser)]
    pub del_repo_preset: Option<String>,

    /// Use TOR proxy for making requests
    #[clap(short, long, value_parser, default_value_t = false)]
    pub tor: bool,

    // Proxy ALL traffic with TOR proxy
    #[clap(short = 'A', long, value_parser, default_value_t = false)]
    pub tor_all: bool,

    /// Change TOR bind address
    #[clap(long, value_parser, default_value = "127.0.0.1")]
    pub tor_bind_address: String,

    /// Change TOR port
    #[clap(long, value_parser, default_value_t = 9050)]
    pub tor_port: i32,
}
