use std::env;

use file_io::Config;

mod tokens;
mod file_io;

fn main() {
    let config = Config::build(env::args());
    file_io::check_file(&config);
}
