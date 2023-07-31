use std::io::stdout;
use std::io::{stdin, Write};

use crate::config::Config;
use crate::model::{Data, UserRecord};

pub fn match_command(mut config: Config, secret_word: String) {
    let mut command = String::new();

    while let Ok(_) = stdin().read_line(&mut command) {
        match command.to_lowercase().trim() {
            "add" => {
                println!("Write your record like: signature domen email password");
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
                                    Ok(_) => println!(
                                        "Record with {} signature was created",
                                        args[0].trim()
                                    ),
                                    Err(err) => println!("{}", err),
                                };
                            }
                            _ => println!("Wrong arguments were provided"),
                        };
                    }
                    Err(_) => println!("Wrong arguments were provided"),
                };
            }
            "remove" => {
                println!("Write signature of your record");
                command = String::new();

                match stdin().read_line(&mut command) {
                    Ok(_) => match config.remove_record(command.trim().to_string(), &secret_word) {
                        Ok(_) => println!("Record with {} signature was deleted", command.trim()),
                        Err(err) => println!("{}", err),
                    },
                    Err(_) => println!("Wrong arguments were provided"),
                };
            }
            "get" => {
                println!("Write signature of your record");
                command = String::new();

                match stdin().read_line(&mut command) {
                    Ok(_) => match config.get_record(command.trim().to_string(), &secret_word) {
                        Ok(record) => println!("{}", record),
                        Err(err) => println!("{}", err),
                    },
                    Err(_) => println!("Wrong arguments were provided"),
                };
            }
            "get_all" => {
                println!("{config}");
            }
            "finish" => {
                stdout().flush().unwrap();
                break;
            }
            _ => println!("Unknown command"),
        };
        command = String::new();
    }
}

pub fn show_command_list() {
    println!(
        "{}Available commands: ADD  |  REMOVE  |  GET  |  GET_ALL  |  FINISH  \n{}",
        create_border(),
        create_border()
    );
}

pub fn create_border() -> String {
    let (x, _) = crossterm::terminal::size().unwrap();
    return "-".repeat(x as usize);
}
