#![crate_name = "french_quiz_game"]
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

/// French Quiz Game
/// Library and utility methods.

/// Read file contents
pub fn read_contents_buffered(path: &str) -> Result<String, Error> {
    let mut file_txt = String::new();
    let readme = File::open(path)?;

    let buffer = BufReader::new(readme);

    for maybe_line in buffer.lines() {
        file_txt.push_str(maybe_line?.as_str());
        file_txt.push('\n'); // Add newline !!!
    }

    Ok(file_txt)
}

/// Takes a filename and path and returns
/// the text file as a String with newline char separating each line.
pub fn read_puzzle_input(path: &str) -> String {
    let input = match read_contents_buffered(path) {
        Ok(file_contents) => {
            println!("Read input file contents successfully!\n\n");
            file_contents
        }
        Err(err) => {
            print!("Error reading input file contents: {:?}\n\n", err);
            panic!();
        }
    };
    input.trim().to_string()
}
