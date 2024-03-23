use crate::{store, utils};

use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn select_config() {
    if !utils::is_fzf_installed() {
        utils::print_outro_cancel(Some("fzf not installed! Please install fzf and try again."));
    }

    utils::ensure_non_empty_config();

    let config = store::get_config();

    let items = config
        .configs
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let mut child = match Command::new("fzf")
        .arg("--prompt= Select Neovim Configuration  ")
        .arg("--height=~50%")
        .arg("--layout=reverse")
        .arg("--border")
        .arg("--exit-0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => utils::print_outro_cancel(Some("Failed to start fzf")),
    };

    if let Some(mut stdin) = child.stdin.take() {
        match stdin.write_all(items.as_bytes()) {
            Ok(_) => {}
            Err(_) => utils::print_outro_cancel(None),
        };
    }

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        match stdout.read_to_string(&mut output) {
            Ok(_) => {}
            Err(_) => utils::print_outro_cancel(None),
        };
    }

    match Command::new("nvim")
        .env(
            "NVIM_APPNAME",
            match config.configs.iter().find(|s| s.name == output.trim()) {
                Some(selected_item) => selected_item.nvim_dir_name.clone(),
                None => utils::print_outro_cancel(None),
            },
        )
        .spawn()
    {
        Ok(mut command) => {
            command.wait().ok();
        }
        Err(_) => utils::print_outro_cancel(Some("Failed to start Neovim")),
    };

    utils::print_outro(None);
}
