use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use clap::{CommandFactory, Parser};
use std::io::{self, Read};

#[derive(Debug)]
struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    file_path: Option<String>,

    #[arg(short='l', long)]
    lines: bool,

    #[arg(short='w', long)]
    words: bool,

    #[arg(short='b', long)]
    bytes: bool,

    #[arg(short='c', long)]
    chars: bool,


}
pub struct CountingResult {
    lines: Option<usize>,
    words: Option<usize>,
    bytes: Option<usize>,
    chars: Option<usize>,
}
impl Display for CountingResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if self.lines.is_some(){
            result.push_str(format!("({} lines)", self.lines.unwrap()).as_str())
        }
        if self.words.is_some(){
            result.push_str(format!(" ({} words)", self.words.unwrap()).as_str())
        }
        if self.bytes.is_some(){
            result.push_str(format!(" ({} bytes)", self.bytes.unwrap()).as_str())
        }
        if self.chars.is_some(){
            result.push_str(format!(" ({} chars)", self.chars.unwrap()).as_str())
        }
        
        f.write_str(result.trim())
    }
}

pub fn run_count(cli: Args) -> Result<CountingResult, Box<dyn Error>>{
    let mut text: String;
    let modifier = cli.lines || cli.bytes || cli.chars || cli.words;
    if cli.file_path.is_some() {
        text = fs::read_to_string(cli.file_path.unwrap())?;
        Ok(
            CountingResult {
                words: if cli.words || modifier == false {Some(text.split_whitespace().count())} else {None},
                lines: if cli.lines || modifier == false {Some(text.lines().count())} else {None},
                chars: if cli.chars || modifier == false {Some(text.chars().count())} else {None},
                bytes: if cli.bytes || modifier == false {Some(text.bytes().count())} else {None}
            }
        )
    }
    else {
        if !atty::is(atty::Stream::Stdin) {
            text = String::new();
            io::stdin().read_to_string(&mut text).unwrap();
            Ok(CountingResult {
                words: if cli.words || modifier == false {Some(text.split_whitespace().count())} else {None},
                lines: if cli.lines || modifier == false {Some(text.lines().count())} else {None},
                chars: if cli.chars || modifier == false {Some(text.chars().count())} else {None},
                bytes: if cli.bytes || modifier == false {Some(text.bytes().count())} else {None}
            })
        } else {
            let mut cmd = Args::command();
            Err(Box::new(MyError(cmd.render_help().to_string())))
        }
    }
}