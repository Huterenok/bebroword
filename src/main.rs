mod cli;
mod config;
mod model;

use std::io::{self};

use config::Config;
use cli::{match_command, show_command_list};

fn main() -> Result<(), io::Error> {
    let (config, secret_word) = Config::initialize();

    show_command_list();

    match_command(config, secret_word);

    Ok(())
}
