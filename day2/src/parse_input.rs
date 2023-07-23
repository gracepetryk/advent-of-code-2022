use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};

use crate::rps::Choice;

pub struct ChoiceReader {
    reader: io::BufReader<File>,
    line_buffer: String
}

impl ChoiceReader {
    pub fn new(file_path: &String) -> Result<ChoiceReader, io::Error> {
        match File::open(file_path) {
            Ok(f) => Ok(ChoiceReader {
                reader: io::BufReader::new(f),
                line_buffer: String::new(),
            }),
            Err(e) => Err(e),
        }
    }
}

impl Iterator for ChoiceReader {
    type Item = Result<(Choice, Choice), Box<dyn Error>>;

    fn next(&mut self) -> Option<Result<(Choice, Choice), Box<dyn Error>>> {
        self.line_buffer.clear();

        let line = match self.reader.read_line(&mut self.line_buffer) {
            Ok(0) => return None, // end of file
            Ok(_) => self.line_buffer.trim_end(),
            Err(e) => return Some(Err(Box::new(e)))
        };

        if line.len() != 3 {
            return Some(Err(format!("line \"{line}\" not 3 chars").into()))
        }

        let (player_choice, opponent_choice) = line.split_at(1); // split on middle space

        let player_choice = match player_choice {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            c => return Some(Err(format!("Unrecognized player choice: {c}").into()))
        };

        let opponent_choice = match opponent_choice {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            c => return Some(Err(format!("Unrecognized opponent choice: {c}").into()))
        };

        Some(Ok((player_choice, opponent_choice)))
    }
}
