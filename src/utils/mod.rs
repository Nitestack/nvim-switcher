mod logger;

use cliclack::{intro, outro, outro_cancel};
use regex::Regex;
use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

use crate::store::get_config;

pub fn is_neovim_installed() -> bool {
    Command::new("nvim").arg("--version").output().is_ok()
}

pub fn is_fzf_installed() -> bool {
    Command::new("fzf").arg("--version").output().is_ok()
}

pub fn print_intro(msg: Option<&str>) {
    intro(
        msg.unwrap_or("Neovim Configuration Switcher")
            .to_uppercase(),
    )
    .unwrap();
}

pub fn print_outro(msg: Option<&str>) {
    outro(logger::get_success_string(
        msg.unwrap_or("Thanks for using Neovim Configuration Switcher!"),
        false,
    ))
    .unwrap();
}

pub fn print_outro_cancel(msg: &str) {
    outro_cancel(logger::get_error_string(msg, true)).unwrap();
    process::exit(0);
}

/// Workaround to enable [console::Emoji]. Disable it with [disable_emojis]
pub fn enable_emojis() {
    match env::var("WT_SESSION") {
        Ok(_) => {}
        Err(_) => env::set_var("WT_SESSION", "nvims-wt-session-hack"),
    }
}

/// Workaround to disable [console::Emoji] after enabling it with [enable_emojis]
pub fn disable_emojis() {
    if env::var("WT_SESSION").unwrap_or("".to_string()) == *"nvims-wt-session-hack" {
        env::remove_var("WT_SESSION");
    };
}

pub fn is_valid_github_url(url: &str) -> bool {
    let re = Regex::new(r"^(https?://github\.com/|git@github\.com:)[^/\s]+/[^/\s]+(\.git)?$")
        .expect("Failed to validate GitHub URL");
    re.is_match(url)
}

pub fn get_nvim_config_dir(path: Option<&str>) -> PathBuf {
    let config_dir = dirs::config_local_dir().expect("Could not get config dir");
    if let Some(path) = path {
        config_dir.join(path)
    } else {
        config_dir
    }
}

pub fn print_empty_configurations() -> bool {
    let config = get_config();
    if config.configs.is_empty() {
        print_intro(None);
        print_outro_cancel("No configurations found! Add a configuration and try again.");
        true
    } else {
        false
    }
}
