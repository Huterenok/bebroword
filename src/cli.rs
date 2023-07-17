use std::io::StdoutLock;
use std::io::{stdin, Write};

use crate::config::Config;
use crate::model::{Data, UserRecord};


pub fn match_command(mut config: Config, secret_word: String, stdout: &mut StdoutLock) {
    let mut command = String::new();

    while let Ok(_) = stdin().read_line(&mut command) {
        match command.to_lowercase().trim() {
            "add" => {
                writeln!(
                    stdout,
                    "Write your record like: signature domen email password"
                )
                .unwrap();
                command = String::new();

                match stdin().read_line(&mut command) {
                    Ok(_) => {
                        let args = command.trim().split(" ").collect::<Vec<&str>>();
                        match args.len() {
                            4 => {
                                let data =
                                    Data::new(args[1].into(), args[2].into(), args[3].into());
                                let record = UserRecord::new(args[0].into(), data);
                                let _res = match config.add_record(record, &secret_word) {
                                    Ok(_) => writeln!(
                                        stdout,
                                        "Record with {} signature was created",
                                        args[0].trim()
                                    ),
                                    Err(err) => writeln!(stdout, "{}", err),
                                };
                            }
                            _ => writeln!(stdout, "Wrong arguments were provided").unwrap(),
                        };
                    }
                    Err(_) => writeln!(stdout, "Wrong arguments were provided").unwrap(),
                };
            }
            "remove" => {
                writeln!(stdout, "Write signature of your record").unwrap();
                command = String::new();

                let _res = match stdin().read_line(&mut command) {
                    Ok(_) => match config.remove_record(command.trim().to_string(), &secret_word) {
                        Ok(_) => writeln!(
                            stdout,
                            "Record with {} signature was deleted",
                            command.trim()
                        ),
                        Err(err) => writeln!(stdout, "{}", err),
                    },
                    Err(_) => writeln!(stdout, "Wrong arguments were provided"),
                };
            }
            "get_all" => {
                writeln!(stdout, "{config}").unwrap();
            }
            "finish" => {
                stdout.flush().unwrap();
                break;
            }
            _ => writeln!(stdout, "Unknown command").unwrap(),
        };
        command = String::new();
    }
}
