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
    ApplyAndroid(AndroidArgs),
    Backup,
    Restore,
    RestoreAndroid(AndroidArgs),
    AddRepo(RepoArgs),
    AddRepoPreset(RepoArgs),
    DelRepo(RepoArgs),
    DelRepoPreset(RepoArgs),
    ListRepos,
    ListDevices,
}

#[derive(Parser, Debug, Clone, PartialEq)]
pub struct SyncArgs {
}

#[derive(Parser, Debug, Clone, PartialEq)]
pub struct RepoArgs {
    /// Specify repository
    pub repo: String
}

#[derive(Parser, Debug, Clone, PartialEq)]
pub struct AndroidArgs {
    /// Specify android device (with device ID) (list devices with `--list-devices`)
    pub device: String
}
