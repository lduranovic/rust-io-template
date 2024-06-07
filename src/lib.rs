use std::io::{self, prelude::*};

pub struct IOTemplate {
    lines: Vec<String>
}

impl IOTemplate {
    pub fn new() -> Self {
        IOTemplate {
            lines: Vec::new()
        }
    }

    pub fn read_everything(&mut self) -> Result<(), io::Error> {
        let mut lock = io::stdin().lock();
        let mut input = String::new();
        loop {
            if let Ok(num_read) = lock.read_line(&mut input) {
                if num_read == 0 {
                    return Ok(());
                } else {
                    self.lines.push(input.to_owned());
                    input.clear();
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Input malformed.")
                );
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
