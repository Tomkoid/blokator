use std::path::Path;
use std::process::exit;

use dirs::{config_dir, home_dir};
use serde::{Deserialize, Serialize};

use crate::actions::{read_file_to_string, write_to_file};
use crate::{Args, HOSTS_FILE, HOSTS_FILE_BACKUP_PATH};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub repositories: Vec<String>,
    pub host_file: String,
    pub restore_file: String,
    pub backup_file: String,
}

impl Config {
    pub fn new(colors: &crate::colors::Colors) -> Config {
        let config_path = Self::config_path();

        if Path::new(&config_path).exists() {
            let toml_str: Result<Config, toml::de::Error> =
                toml::from_str(&read_file_to_string(&config_path).unwrap());

            let toml_str = match toml_str {
                Ok(config) => config,
                Err(_) => {
                    eprintln!(
                        "\n{}error:{} Failed to parse config file: {}",
                        colors.bold_red, colors.reset, config_path
                    );
                    exit(1)
                }
            };

            toml_str
        } else {
            let config = Config {
                repositories: [
                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".into(),
                ]
                .to_vec(),

                host_file: HOSTS_FILE.to_string(),
                restore_file: HOSTS_FILE_BACKUP_PATH.to_string(),
                backup_file: HOSTS_FILE_BACKUP_PATH.to_string(),
            };

            // Write default config
            write_to_file(&config_path, toml::to_string(&config).unwrap());

            config
        }
    }

    pub fn config_path() -> String {
        let xdg_config_home = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or(config_dir().unwrap().to_str().unwrap().into());

        format!("{}/blokator/config.toml", xdg_config_home)
    }
}
