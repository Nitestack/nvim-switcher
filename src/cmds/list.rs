use prettytable::{color, Attr, Cell, Row, Table};

use crate::{store, utils};

pub fn list_configs() {
    utils::ensure_non_empty_config();

    let config = store::get_config();
    let config_dir = utils::get_nvim_config_dir(None);
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Name")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Config Directory"),
        Cell::new("GitHub URL"),
    ]));

    for nvim_config in config.configs.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&nvim_config.name),
            Cell::new(match config_dir.join(&nvim_config.nvim_dir_name).to_str() {
                Some(s) => s,
                None => utils::print_outro_cancel(None),
            }),
            Cell::new(&nvim_config.repo_url),
        ]));
    }

    table.printstd();
}
