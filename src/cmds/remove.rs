use cliclack::{select, spinner};

use crate::{
    store::{get_config, set_config},
    utils::{get_nvim_config_dir, print_empty_configurations, print_intro, print_outro},
};

use std::{env, fs, ops::Add};

pub fn remove_config() {
    if print_empty_configurations() {
        return;
    }
    print_intro(None);

    let config = get_config();

    let selected_key = select("Select Neovim Distribution")
        .items(
            &config
                .configs
                .iter()
                .map(|s| (s.nvim_dir_name.clone(), s.name.clone(), ""))
                .collect::<Vec<_>>(),
        )
        .interact()
        .unwrap();

    let selected_item = &config
        .configs
        .iter()
        .find(|s| s.nvim_dir_name == selected_key)
        .expect("Failed to find selected item");

    let mut spinner = spinner();
    spinner.start(
        format!(
            "Removing config, data, cache and state for {}...",
            selected_item.name
        )
        .as_str(),
    );
    let dir_name = if env::consts::OS == "windows" {
        selected_item.nvim_dir_name.to_string().add("-data")
    } else {
        selected_item.nvim_dir_name.clone()
    };

    fs::remove_dir_all(get_nvim_config_dir(Some(&selected_item.nvim_dir_name))).ok();
    fs::remove_dir_all(
        dirs::data_local_dir()
            .expect("Failed to get data directory")
            .join(&dir_name),
    )
    .ok();
    fs::remove_dir_all({
        let cache_dir = dirs::cache_dir().expect("Failed to get cache directory");
        if env::consts::OS == "windows" {
            dirs::data_local_dir()
                .expect("Failed to get temp directory")
                .join("Temp")
                .join(&dir_name)
        } else {
            cache_dir.join(&dir_name)
        }
    })
    .ok();

    //TODO: As long as `state_dir` returns nothing on Windows, this should work.
    if let Some(state_dir) = dirs::state_dir() {
        fs::remove_dir_all(state_dir.join(&selected_item.nvim_dir_name)).ok();
    }
    spinner.stop("Removed config, data, cache and state");

    spinner.start("Editing user config...");
    let mut new_config = config.clone();
    new_config.configs.swap_remove(
        config
            .configs
            .iter()
            .position(|s| s.nvim_dir_name == selected_item.nvim_dir_name)
            .expect("Failed to find selected item"),
    );
    set_config(new_config, true);
    spinner.stop("User config saved");

    print_outro(Some(
        format!(
            "Removed {} ({})",
            &selected_item.name, &selected_item.repo_url,
        )
        .as_str(),
    ));
}
