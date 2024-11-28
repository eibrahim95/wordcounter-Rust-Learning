use std::process::exit;
use wordcounter::{run_count, Args};
use clap::{ Parser };

fn main() {
    let cli = Args::parse();
    let count = run_count(cli).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });
    println!("{count}")
}
