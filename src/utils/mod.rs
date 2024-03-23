mod logger;

use regex::Regex;
use std::{
    env,
    path::{Component, Path, PathBuf},
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
    cliclack::intro(
        msg.unwrap_or("Neovim Configuration Switcher")
            .to_uppercase(),
    )
    .unwrap();
}

pub fn print_outro(msg: Option<&str>) -> ! {
    cliclack::outro(logger::get_success_string(
        msg.unwrap_or("Thanks for using Neovim Configuration Switcher!"),
        false,
    ))
    .unwrap();
    process::exit(0);
}

pub fn print_outro_cancel(msg: Option<&str>) -> ! {
    cliclack::outro_cancel(logger::get_error_string(
        msg.unwrap_or("An unexpected error occurred. Please try again!"),
        true,
    ))
    .unwrap();
    process::exit(0);
}

/// Workaround to enable [console::Emoji]. Disable it with [disable_emojis]
pub fn enable_emojis() {
    if env::var("WT_SESSION").is_err() {
        env::set_var("WT_SESSION", "nvims-wt-session-hack")
    }
}

/// Workaround to disable [console::Emoji] after enabling it with [enable_emojis]
pub fn disable_emojis() {
    if env::var("WT_SESSION").unwrap_or("".to_string()) == *"nvims-wt-session-hack" {
        env::remove_var("WT_SESSION");
    };
}

pub fn is_valid_github_url(url: &str) -> bool {
    let re = match Regex::new(r"^(https?://github\.com/|git@github\.com:)[^/\s]+/[^/\s]+(\.git)?$")
    {
        Ok(re) => re,
        Err(_) => print_outro_cancel(None),
    };
    re.is_match(url)
}

pub fn is_valid_dir_name(name: &str) -> bool {
    let path = Path::new(name);
    // Check if the path has a single component and that component is not ".."
    path.components().count() == 1
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

pub fn get_nvim_config_dir(path: Option<&str>) -> PathBuf {
    match dirs::config_local_dir() {
        Some(dir) => {
            if let Some(path) = path {
                dir.join(path)
            } else {
                dir
            }
        }
        None => print_outro_cancel(None),
    }
}

pub fn ensure_non_empty_config() {
    let config = get_config();
    if config.configs.is_empty() {
        print_outro_cancel(Some(
            "No configurations found! Add a configuration and try again.",
        ));
    }
}

pub fn setup_ctrl_c_handler() {
    let ctrlc = ctrlc::set_handler(move || {
        println!("{}", logger::get_info_string("Exiting...", true));
        process::exit(0);
    });

    if ctrlc.is_err() {
        println!(
            "{}",
            logger::get_error_string("An unexpected error occurred. Please try again!", true)
        );
        process::exit(0);
    }
}
