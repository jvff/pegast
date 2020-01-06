use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, iter},
};

impl<R: PegAstNode> PegAstNode for Vec<R> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        Ok(iter::from_fn(|| R::parse(input).ok()).collect())
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        let mut string = String::new();

        for element in self {
            string.push_str(element.parsed_string().as_ref());
        }

        Cow::Owned(string)
    }

    fn parsed_string_length(&self) -> usize {
        self.iter().map(PegAstNode::parsed_string_length).sum()
    }

    fn expecting() -> Vec<String> {
        let mut expecting = R::expecting();

        expecting.push("or nothing".to_owned());

        expecting
    }
}
