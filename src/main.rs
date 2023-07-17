mod cli;
mod config;
mod model;

use std::io::{self, Write};

use config::Config;
use cli::match_command;

fn main() -> Result<(), io::Error> {
    let mut stdout = std::io::stdout().lock();
    let (config, secret_word) = Config::initialize();

    writeln!(stdout, "Available commands:\nADD\nREMOVE\nGET_ALL\nFINISH").unwrap();

    match_command(config, secret_word, &mut stdout);

    Ok(())
}
