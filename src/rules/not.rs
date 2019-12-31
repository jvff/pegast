use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, marker::PhantomData},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Not<R: PegAstNode>(PhantomData<R>);

impl<R: PegAstNode> PegAstNode for Not<R> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let parse_result = {
            let mut peek_only = input.peek_only();

            R::parse(&mut peek_only)
        };

        match parse_result {
            Ok(_) => Err(ParseError {
                expected: Self::expecting(),
                position: input.position(),
            }),
            Err(_) => Ok(Not(PhantomData)),
        }
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn expecting() -> Vec<String> {
        let mut expecting = "not ".to_owned();
        let not_expecting = R::expecting();

        match not_expecting.len() {
            0 => expecting.push_str("nothing"),
            1 => expecting.push_str(&not_expecting[0]),
            _ => {
                expecting.push_str("one of:\n");

                for element in not_expecting {
                    expecting.push_str("    - ");
                    expecting.push_str(&element);
                    expecting.push('\n');
                }
            }
        }

        vec![expecting]
    }
}
