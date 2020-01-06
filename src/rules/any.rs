use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, iter},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Any(char);

impl From<char> for Any {
    fn from(character: char) -> Self {
        Any(character)
    }
}

impl PegAstNode for Any {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        input.next().map(Any).ok_or_else(|| ParseError {
            expected: Self::expecting(),
            position: input.position(),
        })
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        Cow::Owned(iter::once(self.0).collect())
    }

    fn parsed_string_length(&self) -> usize {
        1
    }

    fn expecting() -> Vec<String> {
        vec!["any character".to_owned()]
    }
}
