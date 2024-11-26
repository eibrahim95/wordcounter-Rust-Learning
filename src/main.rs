use std::env;
use std::process::exit;
use wordcounter::{count_words, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });
    let count = count_words(config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });
    println!("Count: {count}")
}
