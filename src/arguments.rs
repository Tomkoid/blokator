use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(
    author = "Tomáš Zierl",
    version,
    about,
    long_about = "Easy system-wide adblocker"
)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

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

    /// Restore /etc/hosts backup
    #[clap(short, long, value_parser, default_value_t = false)]
    pub restore: bool,

    /// Restore Android backup of hosts files with ADB (experimental, root required)
    #[clap(long, value_parser, default_value_t = false)]
    pub restore_android: bool,

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

    // Proxy ALL traffic with TOR proxy
    #[clap(short = 't', long, value_parser, default_value_t = false)]
    pub tor: bool,

    /// Change TOR bind address
    #[clap(long, value_parser, default_value = "127.0.0.1")]
    pub tor_bind_address: String,

    /// Change TOR port
    #[clap(long, value_parser, default_value_t = 9050)]
    pub tor_port: i32,
}

#[derive(Parser, Debug, Clone, PartialEq)]
pub enum Commands {
    Sync(SyncArgs),
    Apply,

}

#[derive(Parser, Debug, Clone, PartialEq)]
pub struct SyncArgs {
    sync_type: Option<String>,
}
