use merge::Merge;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone, Merge)]
pub struct Config {
    #[merge(strategy = merge::vec::append)]
    pub configs: Vec<NeovimConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeovimConfig {
    pub name: String,
    pub repo_url: String,
    pub nvim_dir_name: String,
}

fn get_default_config() -> Config {
    Config { configs: vec![] }
}

fn get_config_dir() -> PathBuf {
    match dirs::config_dir() {
        Some(dir) => dir.join("nvims"),
        None => utils::print_outro_cancel(None),
    }
}

fn get_config_path() -> String {
    match get_config_dir().join("config.json").to_str() {
        Some(path) => path.to_string(),
        None => utils::print_outro_cancel(None),
    }
}

pub fn get_config() -> Config {
    let default_config = get_default_config();
    let config_path = get_config_path();

    let config_dir = get_config_dir();

    if !config_dir.exists() && fs::create_dir_all(&config_dir).is_err() {
        utils::print_outro_cancel(Some("Failed to create config directory"))
    }

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config = match fs::read_to_string(&config_path) {
                Ok(config) => config,
                Err(_) => utils::print_outro_cancel(Some("Failed to read config file")),
            };
            match serde_json::from_str(&config) {
                Ok(config) => config,
                Err(_) => utils::print_outro_cancel(Some("Failed to parse config file")),
            }
        }
        Err(_) => match File::create(&config_path) {
            Ok(mut file) => match file.write_all(
                match serde_json::to_string(&default_config) {
                    Ok(config) => config,
                    Err(_) => utils::print_outro_cancel(Some("Failed to serialize config")),
                }
                .as_bytes(),
            ) {
                Ok(_) => default_config,
                Err(_) => utils::print_outro_cancel(Some("Failed to write config file")),
            },
            Err(_) => utils::print_outro_cancel(Some("Failed to create config file")),
        },
    }
}

pub fn set_config(config: Config, override_config: bool) {
    let config_path = get_config_path();
    let mut current_config = get_config();

    if override_config {
        current_config = config;
    } else {
        current_config.merge(config);
    }

    match File::create(config_path) {
        Ok(mut file) => {
            if file
                .write_all(
                    match serde_json::to_string(&current_config) {
                        Ok(config) => config,
                        Err(_) => utils::print_outro_cancel(Some("Failed to serialize config")),
                    }
                    .as_bytes(),
                )
                .is_err()
            {
                utils::print_outro_cancel(Some("Failed to write config file"))
            }
        }
        Err(_) => utils::print_outro_cancel(Some("Failed to create config file")),
    }
}
