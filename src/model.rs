use std::fmt::Display;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};

use crate::cli::create_border;

#[derive(Deserialize, Serialize, Clone)]
pub struct UserRecord {
    pub signature: String,
    pub data: Data,
}

impl UserRecord {
    pub fn new(signature: String, data: Data) -> Self {
        UserRecord { data, signature }
    }
}

impl Display for UserRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nSignature: {}\nDomen: {}\nEmail: {}\nPassword: {}",
            create_border(),
            self.signature,
            self.data.domen,
            self.data.email,
            self.data.password,
        )
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Data {
    pub domen: String,
    pub email: String,
    pub password: String,
}

impl Data {
    pub fn new(domen: String, email: String, password: String) -> Self {
        Data {
            domen,
            email,
            password,
        }
    }

    pub fn encode(&mut self, secret_word: &str) {
        let mc = new_magic_crypt!(secret_word, 256);
        self.password = mc.encrypt_str_to_base64(&self.password);
    }

    pub fn decode(&mut self, secret_word: &str) {
        let mc = new_magic_crypt!(secret_word, 256);
        self.password = mc.decrypt_base64_to_string(&self.password).unwrap();
    }
}
