use std::{
    collections::VecDeque,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
};

/// This is the main structure that this library exports. The user should
/// instantiate the structure and then call methods on it in order to process
/// input.
pub struct IOTemplate {
    /// A deque of lines that the `IOTemplate` object read in.
    lines: VecDeque<String>,
    /// Line we are currently processing from the input.
    current_line: Option<String>,
    /// Tells us which token (word really) we are in, currently.
    cursor: usize,
    /// Tells us at which character (of the given word) we are.
    word_position: usize,
}

impl IOTemplate {
    pub fn new() -> Self {
        IOTemplate {
            lines: VecDeque::new(),
            current_line: None,
            cursor: 0,
            word_position: 0,
        }
    }

    #[allow(dead_code)]
    fn new_with_lines(lines: VecDeque<String>) -> Self {
        IOTemplate {
            lines,
            current_line: None,
            cursor: 0,
            word_position: 0,
        }
    }

    fn read_input<R: BufRead>(reader: R) -> VecDeque<String> {
        reader.lines().map(|line| line.unwrap()).map(|line| line.trim().to_owned()).collect()
    }

    pub fn read_everything(&mut self) {
        let lock = io::stdin().lock();
        self.lines = Self::read_input(lock);
    }

    pub fn read_everything_from_path(
        &mut self,
        path: &Path,
    ) -> Result<(), io::Error> {
        let input_file = File::open(path)?;
        let reader = BufReader::new(input_file);

        self.lines = Self::read_input(reader);
        Ok(())
    }

    pub fn next_line(&mut self) -> Result<String, io::Error> {
        if self.current_line.is_none() {
            if self.lines.is_empty() {
                Err(io::Error::new(io::ErrorKind::NotFound, "No more lines."))
            } else {
                // Reset the other state.
                self.cursor = 0;
                self.word_position = 0;

                // Return the first line.
                let line_to_return = self.lines.pop_front().unwrap();
                Ok(line_to_return)
            }
        } else {
            // Reset the other state.
            self.cursor = 0;
            self.word_position = 0;

            // Return the current line.
            let to_return = self.current_line.to_owned().unwrap();
            self.current_line = None;
            Ok(to_return)
        }
    }

    fn next_current_line(&mut self) -> Result<bool, io::Error> {
        if self.current_line.is_some() {
            Ok(false)
        } else {
            // TODO: Not sure if we really want this here.
            self.cursor = 0;
            self.word_position = 0;
            assert!(self.current_line.is_none());
            if self.lines.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No more lines left.",
                ))
            } else {
                let new_line = self.lines.pop_front();
                self.current_line = new_line;
                Ok(true)
            }
        }
    }

    fn next_token<T>(&mut self) -> Result<T, io::Error>
    where
        T: std::str::FromStr + std::fmt::Debug,
    {
        if self.current_line.is_some() {
            let line: &str = &(self.current_line.as_ref().unwrap());
            let tokens: Vec<&str> = line.split_whitespace().collect();
            assert!(self.cursor <= tokens.len());
            if self.cursor == tokens.len() {
                self.current_line = None;
                let next_current_line_result = self.next_current_line();
                if next_current_line_result.is_ok() {
                    let boolean_result: bool =
                        next_current_line_result.unwrap();
                    if boolean_result {
                        self.next_token()
                    } else {
                        panic!("This should never be reached.");
                    }
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "End of input is likely reached.",
                    ))
                }
            } else {
                let current_token = tokens[self.cursor];
                self.cursor += 1;

                let parsing_result = current_token.parse::<T>();
                match parsing_result {
                    Ok(value) => Ok(value),
                    Err(_) => Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Could not parse the given token.",
                    )),
                }
            }
        } else {
            let next_current_line_result = self.next_current_line();
            if next_current_line_result.is_err() {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No more lines left to process.",
                ))
            } else {
                assert!(next_current_line_result.is_ok());
                let return_value: bool = next_current_line_result.unwrap();
                if return_value {
                    self.next_token()
                } else {
                    panic!("Something went really wrong here.");
                }
            }
        }
    }

    pub fn next_integer(&mut self) -> Result<i64, io::Error> {
        self.next_token::<i64>()
    }

    pub fn next_double(&mut self) -> Result<f64, io::Error> {
        self.next_token::<f64>()
    }

    /// Allows the user to get access to the next character in the input.
    pub fn next_char(&mut self) -> Result<char, io::Error> {
        if self.current_line.is_some() {
            let line: &str = &(self.current_line.as_ref().unwrap());
            let tokens: Vec<&str> = line.split_whitespace().collect();
            assert!(self.cursor <= tokens.len());
            if self.cursor >= tokens.len() {
                self.current_line = None;
                match self.next_current_line() {
                    Ok(bool_value) => {
                        assert!(bool_value);
                        self.next_char()
                    }
                    Err(error) => Err(error),
                }
            } else {
                let current_word: &str = tokens[self.cursor];
                let current_word_len: usize = current_word.len();
                if self.word_position >= current_word_len {
                    self.cursor += 1;
                    self.word_position = 0;
                    self.next_char()
                } else {
                    let next_character: char = current_word.chars().nth(self.word_position).unwrap();
                    self.word_position += 1;
                    Ok(next_character)
                }
            }
        } else {
            self.current_line = None;
            match self.next_current_line() {
                Ok(bool_value) => {
                    assert!(bool_value);
                    self.next_char()
                }
                Err(error) => Err(error),
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_creation() {
        let io_template = IOTemplate::new();

        assert!(io_template.lines.len() == 0);
    }

    #[test]
    fn test_next_line() {
        let mut lines = VecDeque::new();
        lines.push_back("first amazing line\n".to_string());
        lines.push_back("second amazing line\n".to_string());
        lines.push_back("third amazing line\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        let first_line = io_template.next_line().unwrap();
        assert!(first_line == "first amazing line\n".to_string());

        let second_line = io_template.next_line().unwrap();
        assert!(second_line == "second amazing line\n".to_string());

        let third_line = io_template.next_line().unwrap();
        assert!(third_line == "third amazing line\n".to_string());
    }

    #[test]
    fn test_next_token() {
        let mut lines = VecDeque::new();
        lines.push_back("2 4 6 8\n".to_string());
        lines.push_back("1 3 5 7\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        let first_token: i32 = io_template.next_token().unwrap();
        assert!(first_token == 2i32);

        let first_line: String = io_template.next_line().unwrap();
        assert!(first_line == "2 4 6 8\n".to_string());

        let second_token: i32 = io_template.next_token().unwrap();
        assert!(second_token == 1i32);

        let third_token: i32 = io_template.next_token().unwrap();
        assert!(third_token == 3i32);
    }

    #[test]
    fn test_next_integer() {
        let mut lines = VecDeque::new();
        lines.push_back("first line\n".to_string());
        lines.push_back("1 2 3 4\n".to_string());
        lines.push_back("second line\n".to_string());
        lines.push_back("5 6 7 8\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        let bad_integer_result = io_template.next_integer();
        assert!(bad_integer_result.is_err());

        let _ = io_template.next_current_line();

        let next_line = io_template.next_line().unwrap();
        println!("This is the current line: {next_line}");
    }

    #[test]
    fn test_next_char() {
        let mut lines = VecDeque::new();
        lines.push_back("first line\n".to_string());
        lines.push_back("second line\n".to_string());
        lines.push_back("sh\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        let first_character: char = io_template.next_char().unwrap();
        assert!(first_character == 'f');
        let second_character: char = io_template.next_char().unwrap();
        assert!(second_character == 'i');

        // Skip to the next line to make things more interesting.
        let first_line = io_template.next_line().unwrap();
        assert!(first_line == "first line\n".to_string());

        // Skip the first two characters now.
        let _ = io_template.next_char();
        let _ = io_template.next_char();

        let third_character: char = io_template.next_char().unwrap();
        assert!(third_character == 'c');

        // Skip to the last line.
        let _ = io_template.next_line();

        // Skip the first two characters.
        let first = io_template.next_char().unwrap();
        assert!(first == 's');
        let second = io_template.next_char().unwrap();
        assert!(second == 'h');
        let error = io_template.next_char();
        assert!(error.is_err());

        // Start off again.
        let mut lines = VecDeque::new();
        lines.push_back("fst line\n".to_string());
        lines.push_back("z\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        // Let's try to use `next_char()` to exhaust the first line fully. Make
        // sure here that all the characters are consistent.
        let first = io_template.next_char().unwrap();
        assert!(first == 'f');
        let second = io_template.next_char().unwrap();
        assert!(second == 's');
        let third = io_template.next_char().unwrap();
        assert!(third == 't');

        // Go through the second word.
        let _ = io_template.next_char();
        let _ = io_template.next_char();
        let _ = io_template.next_char();
        let _ = io_template.next_char();

        // First character on the second line makes sense.
        let first_character_second_line = io_template.next_char().unwrap();
        assert!(first_character_second_line == 'z');

        // You cannot call this anymore.
        let error = io_template.next_char();
        assert!(error.is_err());
    }

    // TODO: Add some tests for mixing the functions together. This is where I
    // think problems might pop up.
}
