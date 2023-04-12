pub use std::{path::Path, io::Write, fs};
pub use std::process::exit;

pub use spinners::Spinner;
pub use crate::SPINNER_TYPE;

pub use crate::{get_data_dir, read::read_file_to_string, colors::Colors, messages::Messages, sync::sync, write::write_to_file, arguments::Args};
pub use crate::copy::copy;

pub mod sync;
pub mod apply;
pub mod backup;
pub mod restore;
pub mod add_repo;
pub mod add_repo_preset;
pub mod del_repo;
pub mod del_repo_preset;
pub mod list_repos;

pub mod apply_android;
pub mod restore_android;
pub mod list_devices;
