pub mod input;
pub mod rules;

#[cfg(feature = "derive")]
pub use pegast_derive::PegAstNode;
use {
    crate::input::Input,
    std::{
        borrow::Cow,
        error::Error,
        fmt::{self, Display, Formatter},
        ops::Deref,
    },
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

pub trait PegAstNode: Sized {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError>;
    fn parsed_string(&self) -> Cow<'_, str>;
    fn expecting() -> Vec<String>;

    fn parsed_string_length(&self) -> usize {
        self.parsed_string().len()
    }
}

impl<T> PegAstNode for Box<T>
where
    T: PegAstNode,
{
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let inner = T::parse(input)?;

        Ok(Box::new(inner))
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        self.deref().parsed_string()
    }

    fn parsed_string_length(&self) -> usize {
        self.deref().parsed_string_length()
    }

    fn expecting() -> Vec<String> {
        T::expecting()
    }
}
