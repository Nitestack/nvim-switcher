use merge::Merge;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
        None => panic!("Could not get config dir"),
    }
}

fn get_config_path() -> String {
    match get_config_dir().join("config.json").to_str() {
        Some(path) => path.to_string(),
        None => panic!("Could not get config dir"),
    }
}

pub fn get_config() -> Config {
    let default_config = get_default_config();
    let config_path = get_config_path();

    let config_dir = get_config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config dir");
    }

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config = fs::read_to_string(&config_path).expect("Could not read config file");
            serde_json::from_str(&config).unwrap_or(default_config)
        }
        Err(_) => {
            File::create(&config_path)
                .expect("Could not create config file")
                .write_all(
                    serde_json::to_string(&default_config)
                        .expect("Could not default serialize config")
                        .as_bytes(),
                )
                .expect("Could not write config file");
            default_config
        }
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

    File::create(config_path)
        .expect("Could not create config file")
        .write_all(
            serde_json::to_string(&current_config)
                .expect("Could not serialize config")
                .as_bytes(),
        )
        .expect("Could not write config file");
}
