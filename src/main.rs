mod cli;
mod cmds;
mod store;
mod utils;

fn main() -> std::io::Result<()> {
    utils::setup_ctrl_c_handler();
    utils::enable_emojis();
    cli::run_cli();
    utils::disable_emojis();
    Ok(())
}
