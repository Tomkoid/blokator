pub use std::process::exit;
pub use std::{fs, io::Write, path::Path};

pub use crate::SPINNER_TYPE;
pub use spinners::Spinner;

pub use crate::copy::copy;
pub use crate::{
    arguments::Args, colors::Colors, get_data_dir, messages::Messages, read::read_file_to_string,
    sync::sync, write::write_to_file,
};

pub mod add_repo_preset;
pub mod apply;
pub mod backup;
pub mod del_repo;
pub mod del_repo_preset;
pub mod list_repos;
pub mod restore;
pub mod sync;

pub mod apply_android;
pub mod list_devices;
pub mod restore_android;
