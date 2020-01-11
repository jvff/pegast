use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, marker::PhantomData},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Peek<R: PegAstNode>(PhantomData<R>);

impl<R: PegAstNode> PegAstNode for Peek<R> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let mut peek_only = input.peek_only();

        R::parse(&mut peek_only)?;

        Ok(Peek(PhantomData))
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn parsed_string_length(&self) -> usize {
        0
    }

    fn expecting() -> Vec<String> {
        R::expecting()
    }
}
