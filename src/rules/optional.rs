use {
    crate::{input::Input, ParseError, PegAstNode},
    std::borrow::Cow,
};

impl<R: PegAstNode> PegAstNode for Option<R> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        Ok(R::parse(input).ok())
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        match self {
            Some(node) => node.parsed_string(),
            None => Cow::Borrowed(""),
        }
    }

    fn parsed_string_length(&self) -> usize {
        self.as_ref()
            .map(PegAstNode::parsed_string_length)
            .unwrap_or(0)
    }

    fn expecting() -> Vec<String> {
        let mut expecting = R::expecting();

        expecting.push("or nothing".to_owned());

        expecting
    }
}
