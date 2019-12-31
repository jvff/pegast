pub mod input;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct ParseError {
    pub expected: Vec<String>,
    pub position: usize,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "Failed to parse input at {}", self.position)?;
        write!(formatter, "  Expected:")?;

        match self.expected.len() {
            0 => writeln!(formatter, " nothing")?,
            1 => writeln!(formatter, " {}", self.expected[0])?,
            _ => {
                writeln!(formatter, " one of:")?;

                for element in &self.expected {
                    writeln!(formatter, "    - {}", element)?;
                }
            }
        }

        Ok(())
    }
}

impl Error for ParseError {}
