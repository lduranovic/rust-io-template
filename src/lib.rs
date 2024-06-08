use std::{
    collections::VecDeque, fs::File, io::{self, prelude::*, BufReader}, path::Path
};

pub struct IOTemplate {
    lines: VecDeque<String>,
    current_line: Option<String>,
}

impl IOTemplate {
    pub fn new() -> Self {
        IOTemplate { lines: VecDeque::new(), current_line: None }
    }

    pub fn read_everything(&mut self) -> Result<(), io::Error> {
        let mut lock = io::stdin().lock();
        let mut input = String::new();
        loop {
            if let Ok(num_read) = lock.read_line(&mut input) {
                if num_read == 0 {
                    return Ok(());
                } else {
                    self.lines.push_back(input.to_owned());
                    input.clear();
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Input malformed.",
                ));
            }
        }
    }

    pub fn read_everything_from_path(&mut self, path: &Path) -> Result<(), io::Error> {
        let input_file = File::open(path)?;
        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            let line = line?;
            self.lines.push_back(line);
        }

        Ok(())
    }

    pub fn next_line(&mut self) -> Option<String> {
        if self.current_line.is_none() {
            if self.lines.is_empty() {
                None
            } else {
                self.lines.pop_front()
            }
        } else {
            let to_return = self.current_line.to_owned();
            self.current_line = None;
            to_return
        }
    }

    fn next_current_line(&mut self) -> Result<bool, io::Error> {
        if self.current_line.is_some() {
            Ok(false)
        } else {
            assert!(self.current_line.is_none());
            if self.lines.is_empty() {
                // There are no lines to return.
                Ok(false)
            } else {
                // There is a line we could return.
                let new_line = self.lines.pop_front();
                self.current_line = new_line.to_owned();
                Ok(true)
            }
        }
    }

    pub fn next_integer(&mut self) -> Option<i64> {
        if self.lines.is_empty() {
            None
        } else {
            let current_line: String = self.next_line().unwrap();
            let mut tokens: VecDeque<String> = current_line
                .trim()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect();
            if tokens.len() == 0 {
                // In this case, this line should be removed from the
                // collection of lines anyways. So no repair is needed.
                None
            } else {
                // Get the first token.
                let first_token = tokens.pop_front();

                // Repair the collection of lines.
                let new_line = tokens.iter().cloned().collect::<Vec<String>>().join(" ");
                self.lines.push_front(new_line);

                // Work on the first token. Try to return a number if it is a
                // number.
                if let Some(num) = first_token {
                    if let Ok(to_return) = num.parse::<i64>() {
                        Some(to_return)
                    } else {
                        // TODO: There should be a better check here. If this
                        // does not go through, that means you messed up in
                        // terms of reading the number.
                        None
                    }
                } else {
                    None
                }
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
}
