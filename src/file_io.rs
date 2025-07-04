use crate::tokens;
use std::{fs::File, io::Read};

pub fn check_file(config: &Config) {
    let mut file = match File::open(&config.file_path) {
        Ok(f) => f,
        Err(e) => rspell::report_error(&format!("Couldn't open {}: {e})", config.file_path)),
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        rspell::report_error(&format!("Couldn't read from {}: {e}", config.file_path));
    }
    let dictionary = rspell::load_dictionary();

    for token in tokens::tokenize(&contents) {
        if let Some(message) = token.format(&dictionary) {
            println!("{message}");
        }
    }
}

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        // skip file name
        args.next();
        if let Some(file_path) = args.next() {
            Self { file_path }
        } else {
            rspell::report_error(&format!("Could not read args"));
        }
    }
}
