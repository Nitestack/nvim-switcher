use colored::*;

use cliclack::log;

pub fn get_error_string(err: &str, with_prefix: bool) -> String {
    format!(
        "{}{}",
        match with_prefix {
            true => format!("{} ", " ERROR ".on_red().black()),
            false => "".to_string(),
        },
        err.red()
    )
}

pub fn get_success_string(msg: &str, with_prefix: bool) -> String {
    format!(
        "{}{}",
        match with_prefix {
            true => format!("{} ", " SUCCESS ".on_green().black()),
            false => "".to_string(),
        },
        msg.green()
    )
}

pub fn get_info_string(msg: &str, with_prefix: bool) -> String {
    format!(
        "{}{}",
        match with_prefix {
            true => format!("{} ", " INFO ".on_cyan().black()),
            false => "".to_string(),
        },
        msg.cyan()
    )
}

pub fn error(err: &str, with_prefix: bool) {
    log::error(get_error_string(err, with_prefix)).unwrap();
}

pub fn success(msg: &str, with_prefix: bool) {
    log::success(get_success_string(msg, with_prefix)).unwrap();
}

pub fn info(msg: &str, with_prefix: bool) {
    log::info(get_info_string(msg, with_prefix)).unwrap();
}
