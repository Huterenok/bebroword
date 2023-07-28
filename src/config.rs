use std::fmt::Display;
use std::fs::{self, File};
use std::fs::OpenOptions;
use std::io::{stdin, BufReader};

use crate::model::UserRecord;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub control_word: String,
    pub records: Vec<UserRecord>,
}

const PATH: &str = "./config/config.json";

impl Config {
    pub fn initialize() -> (Self, String) {
        let config = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(PATH)
            .unwrap();
        let reader = BufReader::new(config);
        let mut config = Config::default();
        let mut secret_word = String::new();

        match serde_json::from_reader::<BufReader<File>, Config>(reader) {
            Ok(data) => {
                println!("Welcome to Bebroword. Enter your secret_word to start our work!",);

                let mut input = String::new();
                while let Ok(_) = stdin().read_line(&mut input) {
                    let mc = new_magic_crypt!(&input, 256);
                    let control_word = mc.decrypt_base64_to_string(&data.control_word);

                    match control_word {
                        Ok(_) => {
                            println!("Nice! We are ready to work!");
                            break;
                        }
                        Err(_) => {
                            println!("It is wrong secret_word. Enter it again if you know it)))");
                            input = String::new();
                            continue;
                        }
                    }
                }

                secret_word = input;
                config = data
            }
            Err(_) => {
                println!("Welcome to Bebroword. Enter your secret_word to start our work!");

                let mut data = String::new();
                while let Ok(_) = stdin().read_line(&mut data) {
                    match data.len() {
                        0 => {
                            println!("Your secret_word is too short)) Try to enter another one");
                            data = String::new();
                            continue;
                        }
                        _ => {
                            let mc = new_magic_crypt!(&data, 256);
                            let control_word = mc.encrypt_str_to_base64("bebra");

                            config = Config::new(control_word);
                            secret_word = data;

                            fs::write(PATH, json!(config).to_string()).unwrap();
                            println!(
                                "Config was successfully saved. Welcome, your secret word is {}",
                                &secret_word
                            );
                            break;
                        }
                    }
                }
            }
        };

        config.decode(&secret_word);

        (config, secret_word)
    }

    pub fn new(control_word: String) -> Self {
        Config {
            control_word,
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: UserRecord, secret_word: &str) -> Result<(), String> {
        if let Some(_) = self
            .records
            .iter()
            .position(|item| item.signature == record.signature)
        {
            return Err("Record with this signature already exists".to_string());
        };

        self.records.push(record);

        self.write_all(secret_word);
        Ok(())
    }

    pub fn remove_record(&mut self, signature: String, secret_word: &str) -> Result<(), String> {
        if let Some(index) = self
            .records
            .iter()
            .position(|record| record.signature == signature)
        {
            self.records.remove(index);
            self.write_all(secret_word);
            Ok(())
        } else {
            Err("Can't find record with this signature".to_string())
        }
    }

    fn write_all(&self, secret_word: &str) {
        let mut json_config = self.clone();
        json_config.records.iter_mut().for_each(|record| {
            record.data.encode(secret_word);
        });

        fs::write(PATH, json!(json_config).to_string()).unwrap();
    }

    fn decode(&mut self, secret_word: &str) {
        self.records.iter_mut().for_each(|record| {
            record.data.decode(&secret_word);
        });
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let records = self
            .records
            .iter()
            .fold("".into(), |mut acc: String, record| {
                acc.push_str(&format!("{}\n", record));
                acc
            });
        write!(f, "\nYour records: \n{}", records)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            control_word: "".to_string(),
            records: vec![],
        }
    }
}
