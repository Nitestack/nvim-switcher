use crate::{
    cmds::*,
    utils::{is_neovim_installed, print_outro_cancel},
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nvims", author = "Nitestack", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new Neovim configuration
    Add {},
    /// Remove a Neovim configuration
    Remove {},
    /// Configure a Neovim configuration
    Config {},
    /// List all Neovim configurations
    List {},
}

pub fn run_cli() {
    let cli = Cli::parse();

    if !is_neovim_installed() {
        print_outro_cancel("Neovim not installed! Please install Neovim and try again.");
        return;
    }

    match &cli.command {
        Some(Commands::Add {}) => add::add_config(),
        Some(Commands::Remove {}) => remove::remove_config(),
        Some(Commands::Config {}) => {}
        Some(Commands::List {}) => list::list_configs(),
        None => select::select_config(),
    }
}
