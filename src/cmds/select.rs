// use cliclack::select;

use crate::{
    store::get_config,
    utils::{
        is_fzf_installed, print_empty_configurations, print_intro, print_outro, print_outro_cancel,
    },
};

use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn select_config() {
    if !is_fzf_installed() {
        print_intro(None);
        print_outro_cancel("fzf not installed! Please install fzf and try again.");
        return;
    }

    if print_empty_configurations() {
        return;
    }

    let config = get_config();

    let items = config
        .configs
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let mut child = Command::new("fzf")
        .arg("--prompt= Select Neovim Distribution  ")
        .arg("--height=~50%")
        .arg("--layout=reverse")
        .arg("--border")
        .arg("--exit-0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn fzf process");

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(items.as_bytes()).unwrap();
    }

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut output).unwrap();
    }

    Command::new("nvim")
        .env(
            "NVIM_APPNAME",
            config
                .configs
                .iter()
                .find(|s| s.name == output.trim())
                .expect("Failed to find selected item")
                .nvim_dir_name
                .clone(),
        )
        .spawn()
        .unwrap()
        .wait()
        .ok();

    print_outro(None);
}
