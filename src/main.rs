mod cli;
mod cmds;
mod store;
mod utils;

extern crate core;

use crate::{
    cli::run_cli,
    utils::{disable_emojis, enable_emojis},
};

fn main() {
    enable_emojis();
    run_cli();
    disable_emojis();
}
