use cliclack;

use crate::{store, utils};

use std::{env, fs, ops::Add};

pub fn remove_config() {
    utils::ensure_non_empty_config();

    let config = store::get_config();

    let selected_key = match cliclack::select("Select Neovim Configuration")
        .items(
            &config
                .configs
                .iter()
                .map(|s| (s.nvim_dir_name.clone(), s.name.clone(), ""))
                .collect::<Vec<_>>(),
        )
        .interact()
    {
        Ok(selected_key) => selected_key,
        Err(_) => utils::print_outro_cancel(Some("Failed to get selected configuration")),
    };

    let selected_item = match config
        .configs
        .iter()
        .find(|s| s.nvim_dir_name == selected_key)
    {
        Some(selected_item) => selected_item,
        None => utils::print_outro_cancel(None),
    };

    let mut spinner = cliclack::spinner();
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

    fs::remove_dir_all(utils::get_nvim_config_dir(Some(
        &selected_item.nvim_dir_name,
    )))
    .ok();
    fs::remove_dir_all(match dirs::data_local_dir() {
        Some(data_local_dir) => data_local_dir.join(&dir_name),
        None => utils::print_outro_cancel(None),
    })
    .ok();
    fs::remove_dir_all({
        if env::consts::OS == "windows" {
            match dirs::data_local_dir() {
                Some(data_local_dir) => data_local_dir.join("Temp").join(dir_name),
                None => utils::print_outro_cancel(None),
            }
        } else {
            let cache_dir = match dirs::cache_dir() {
                Some(cache_dir) => cache_dir,
                None => utils::print_outro_cancel(None),
            };
            cache_dir.join(dir_name)
        }
    })
    .ok();

    // INFO: As long as `state_dir` returns nothing on Windows, this should work.
    if let Some(state_dir) = dirs::state_dir() {
        fs::remove_dir_all(state_dir.join(&selected_item.nvim_dir_name)).ok();
    }
    spinner.stop("Removed config, data, cache and state");

    spinner.start("Editing user config...");
    let mut new_config = config.clone();
    new_config.configs.swap_remove(
        match config
            .configs
            .iter()
            .position(|s| s.nvim_dir_name == selected_item.nvim_dir_name)
        {
            Some(position) => position,
            None => utils::print_outro_cancel(None),
        },
    );
    store::set_config(new_config, true);
    spinner.stop("User config saved");

    utils::print_outro(Some(
        format!(
            "Removed {} ({})",
            selected_item.name, selected_item.repo_url,
        )
        .as_str(),
    ));
}
