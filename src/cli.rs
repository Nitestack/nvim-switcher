use crate::{cmds, utils};
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
    utils::print_intro(None);
    let cli = Cli::parse();

    if !utils::is_neovim_installed() {
        utils::print_outro_cancel(Some(
            "Neovim not installed! Please install Neovim and try again.",
        ));
    }

    match &cli.command {
        Some(Commands::Add {}) => cmds::add::add_config(),
        Some(Commands::Remove {}) => cmds::remove::remove_config(),
        Some(Commands::Config {}) => {}
        Some(Commands::List {}) => cmds::list::list_configs(),
        None => cmds::select::select_config(),
    }
}
