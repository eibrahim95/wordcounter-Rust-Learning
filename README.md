# Word counter
This is a simple word counter that reads a text file or text from the command line and counts the number of words in it.

## Installation
```bash
cargo install --path .
```

## Usage: 
```bash
wordcounter -F "path/to/file.txt"
wordcounter "This is a text"
```

## Development
```bash
cargo run -- -F "path/to/file.txt"
cargo run -- "This is a text"
```
