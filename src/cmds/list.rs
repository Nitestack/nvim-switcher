use prettytable::{color, Attr, Cell, Row, Table};

use crate::{store::get_config, utils::get_nvim_config_dir};

pub fn list_configs() {
    let config = get_config();
    let config_dir = get_nvim_config_dir(None);
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
            Cell::new(
                config_dir
                    .join(&nvim_config.nvim_dir_name)
                    .to_str()
                    .unwrap(),
            ),
            Cell::new(&nvim_config.repo_url),
        ]));
    }

    table.printstd();
}
