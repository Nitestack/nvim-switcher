use cliclack;
use std::process::Command;

use crate::{store, utils};

pub fn add_config() {
    let configs_dir = utils::get_nvim_config_dir(None);

    let name: String = match cliclack::input("Name")
        .placeholder("Configuration Name")
        .validate(|input: &String| {
            if store::get_config()
                .configs
                .iter()
                .any(|nvim_config| nvim_config.name.to_lowercase() == *input.to_lowercase())
            {
                Err("Configuration name already exists")
            } else {
                Ok(())
            }
        })
        .interact()
    {
        Ok(name) => name,
        Err(_) => utils::print_outro_cancel(Some("Failed to get configuration name")),
    };
    let repo_url: String = match cliclack::input("GitHub Repository URL")
        .placeholder("https://github.com/user/repo")
        .validate(|input: &String| {
            if !utils::is_valid_github_url(input) {
                Err("URL must be a valid GitHub repository URL")
            } else {
                Ok(())
            }
        })
        .interact()
    {
        Ok(repo_url) => repo_url,
        Err(_) => utils::print_outro_cancel(Some("Failed to get GitHub Repository URL")),
    };
    let nvim_dir_name: String = match cliclack::input(format!(
        "{} ({})",
        "Neovim Directory Name",
        match configs_dir.join("<configuration-name>").to_str() {
            Some(s) => s,
            None => utils::print_outro_cancel(None),
        }
    ))
    .placeholder("configuration-name")
    .validate(|input: &String| {
        if store::get_config()
            .configs
            .iter()
            .any(|nvim_config| nvim_config.nvim_dir_name == *input)
        {
            Err("Directory name already exists")
        } else if !utils::is_valid_dir_name(input) {
            Err("Directory name is not valid")
        } else {
            Ok(())
        }
    })
    .interact()
    {
        Ok(nvim_dir_name) => nvim_dir_name,
        Err(_) => utils::print_outro_cancel(Some("Failed to get Neovim Directory Name")),
    };

    let mut spinner = cliclack::spinner();
    spinner.start(format!("Cloning {} from '{}'...", name, repo_url).as_str());
    if Command::new("git")
        .arg("clone")
        .arg(&repo_url)
        .arg(configs_dir.join(&nvim_dir_name))
        .arg("--depth")
        .arg("1")
        .output()
        .is_err()
    {
        utils::print_outro_cancel(Some("Failed to clone repository"))
    }
    spinner.stop(format!("Cloned {} from '{}'", name, repo_url).as_str());

    spinner.start("Editing user config...");
    store::set_config(
        store::Config {
            configs: vec![store::NeovimConfig {
                name: name.clone(),
                repo_url: repo_url.clone(),
                nvim_dir_name: nvim_dir_name.clone(),
            }],
        },
        false,
    );
    spinner.stop("User config saved");

    utils::print_outro(Some(
        format!(
            "Added {} ({}) to '{}'",
            name,
            repo_url,
            match configs_dir.join(nvim_dir_name).to_str() {
                Some(s) => s,
                None => utils::print_outro_cancel(None),
            }
        )
        .as_str(),
    ));
}
