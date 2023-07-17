mod cli;
mod config;
mod model;

use std::io::{self, stdin, Write};

use config::Config;
use model::{Data, UserRecord};

fn main() -> Result<(), io::Error> {
    let mut stdout = std::io::stdout().lock();
    let (mut config, secret_word) = Config::initialize();

    let mut buffer = String::new();

    writeln!(stdout, "Available commands:\nADD\nREMOVE\nGET_ALL\nFINISH").unwrap();

    while let Ok(_) = stdin().read_line(&mut buffer) {
        match buffer.trim() {
            "ADD" => {
                writeln!(
                    stdout,
                    "Write your record like: signature domen email password"
                )
                .unwrap();
								buffer = String::new();

                match stdin().read_line(&mut buffer) {
                    Ok(_) => {
                        let args = buffer.trim().split(" ").collect::<Vec<&str>>();
                        match args.len() {
                            4 => {
                                let data =
                                    Data::new(args[1].into(), args[2].into(), args[3].into());
                                let record = UserRecord::new(args[0].into(), data);
																let _res = match config.add_record(record, &secret_word) {
																		Ok(_) => writeln!(stdout, "Record with {} signature was created", args[0].trim()),
																		Err(err) => writeln!(stdout, "{}", err)
																};
                            }
                            _ => writeln!(stdout, "Wrong arguments were provided").unwrap(),
                        };
                    }
                    Err(_) => writeln!(stdout, "Wrong arguments were provided").unwrap(),
                };
            }
            "REMOVE" => {
                writeln!(stdout, "Write signature of your record").unwrap();
								buffer = String::new();

                let _res = match stdin().read_line(&mut buffer) {
                    Ok(_) => match config.remove_record(buffer.trim().to_string(), &secret_word) {
                        Ok(_) => writeln!(stdout, "Record with {} signature was deleted", buffer.trim()),
                        Err(err) => writeln!(stdout, "{}", err),
                    },
                    Err(_) => writeln!(stdout, "Wrong arguments were provided"),
                };
            }
            "GET_ALL" => {
                writeln!(stdout, "{config}").unwrap();
            }
            "FINISH" => {
                stdout.flush().unwrap();
                break;
            }
            _ => writeln!(stdout, "Unknown command").unwrap(),
        };
				buffer = String::new();
    }

    Ok(())
}
