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
        reader.lines().map(|line| line.unwrap()).collect()
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
                let line_to_return = self.lines.pop_front().unwrap();
                Ok(line_to_return)
            }
        } else {
            let to_return = self.current_line.to_owned().unwrap();
            self.current_line = None;
            Ok(to_return)
        }
    }

    fn next_current_line(&mut self) -> Result<bool, io::Error> {
        if self.current_line.is_some() {
            Ok(false)
        } else {
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

    pub fn next_char(&mut self) -> Result<char, io::Error> {
        Ok('a')
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

    // TODO: Make this test more comprehensive.
    #[test]
    fn test_next_char() {
        let mut lines = VecDeque::new();
        lines.push_back("first line\n".to_string());
        lines.push_back("second line\n".to_string());
        lines.push_back("1 2 3 4\n".to_string());
        lines.push_back("5 6 7 8\n".to_string());

        let mut io_template = IOTemplate::new_with_lines(lines);

        let first_character: char = io_template.next_char().unwrap();
        assert!(first_character == 'f');
    }
}
