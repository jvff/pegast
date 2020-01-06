use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, iter},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OneOrMore<T> {
    pub head: T,
    pub tail: Vec<T>,
}

impl<T> OneOrMore<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.head).chain(self.tail.iter())
    }
}

impl<T> PegAstNode for OneOrMore<T>
where
    T: PegAstNode,
{
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let (new_position, result) = {
            let mut peek_input = input.peek_only();
            let head = T::parse(&mut peek_input)?;
            let tail = Vec::parse(&mut peek_input)?;

            (peek_input.position(), OneOrMore { head, tail })
        };

        input.advance_to(new_position);

        Ok(result)
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        if self.tail.is_empty() {
            self.head.parsed_string()
        } else {
            let mut string = self.head.parsed_string().into_owned();

            for item in &self.tail {
                string.push_str(&item.parsed_string());
            }

            Cow::Owned(string)
        }
    }

    fn parsed_string_length(&self) -> usize {
        self.head.parsed_string_length() + self.tail.parsed_string_length()
    }

    fn expecting() -> Vec<String> {
        T::expecting()
    }
}
