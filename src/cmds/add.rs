use cliclack::{input, spinner};
use std::process::Command;

use crate::store::{get_config, set_config, Config, NeovimConfig};
use crate::utils::{get_nvim_config_dir, is_valid_github_url, print_intro, print_outro};

pub fn add_config() {
    let configs_dir = get_nvim_config_dir(None);

    print_intro(None);

    let name: String = input("Name")
        .placeholder("Configuration Name")
        .validate(|input: &String| {
            if get_config()
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
        .unwrap();
    let repo_url: String = input("GitHub Repository URL")
        .placeholder("https://github.com/user/repo")
        .validate(|input: &String| {
            if !is_valid_github_url(input) {
                Err("URL must be a valid GitHub repository URL")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();
    let nvim_dir_name: String = input(format!(
        "{} ({})",
        "Neovim Directory Name",
        configs_dir.join("<configuration-name>").to_str().unwrap()
    ))
    .placeholder("configuration-name")
    .validate(|input: &String| {
        if get_config()
            .configs
            .iter()
            .any(|nvim_config| nvim_config.nvim_dir_name == *input)
        {
            Err("Directory name already exists")
        } else {
            Ok(())
        }
    })
    .interact()
    .unwrap();

    let mut spinner = spinner();
    spinner.start(format!("Cloning {} from '{}'...", name, repo_url).as_str());
    Command::new("git")
        .arg("clone")
        .arg(&repo_url)
        .arg(configs_dir.join(&nvim_dir_name))
        .arg("--depth")
        .arg("1")
        .output()
        .expect("Failed to clone repository");
    spinner.stop(format!("Cloned {} from '{}'", name, repo_url).as_str());

    spinner.start("Editing user config...");
    set_config(
        Config {
            configs: vec![NeovimConfig {
                name: name.clone(),
                repo_url: repo_url.clone(),
                nvim_dir_name: nvim_dir_name.clone(),
            }],
        },
        false,
    );
    spinner.stop("User config saved");

    print_outro(Some(
        format!(
            "Added {} ({}) to '{}'",
            name,
            repo_url,
            configs_dir.join(nvim_dir_name).to_str().unwrap()
        )
        .as_str(),
    ));
}
