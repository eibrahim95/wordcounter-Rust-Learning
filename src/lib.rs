use std::error::Error;
use std::fs;

const USAGE: &str = "Usage:\n    wordcounter \"STRING\" OR wordcounter -f /path/to/file";
#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    text: Option<String>,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let operation_mode = match args.next() {
            Some(arg) => arg,
            None => return Err(USAGE)
        };
        let operation_mode = match operation_mode.as_str() {
            "-f" => "file",
            text => text
        };
        match operation_mode { 
            "file" => match args.next() {
                Some(arg) => Ok(Self {
                    file: Some(arg.to_string()),
                    text: None
                }),
                None => Err(USAGE)
            },
            text => Ok(Self{
                file: None,
                text: Some(text.to_string())
            })
        }
    }
}
pub fn count_words(config: Config) -> Result<usize, Box<dyn Error>>{
    let text: String;
    if config.file.is_some() {
        text = fs::read_to_string(config.file.unwrap())?;
    }
    else {
        text = config.text.unwrap();
    }
    Ok(text.split_whitespace().count())
}