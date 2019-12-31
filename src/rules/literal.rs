use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, marker::PhantomData},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Literal<V: LiteralValue>(PhantomData<V>);

impl<V: LiteralValue> Literal<V> {
    pub fn new() -> Self {
        Literal(PhantomData)
    }
}

impl<V: LiteralValue> PegAstNode for Literal<V> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        if input.consume(V::LITERAL) {
            Ok(Literal(PhantomData))
        } else {
            Err(ParseError {
                expected: Self::expecting(),
                position: input.position(),
            })
        }
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        Cow::Borrowed(V::LITERAL)
    }

    fn expecting() -> Vec<String> {
        vec![V::LITERAL.to_owned()]
    }
}

pub trait LiteralValue {
    const LITERAL: &'static str;
}

#[macro_export]
macro_rules! literals {
    ( $name:ident : $literal:expr ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {}

        literals!(@impl $name : $literal);
    };

    ( pub $name:ident : $literal:expr ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {}

        literals!(@impl $name : $literal);
    };

    ( $name:ident : $literal:expr , $( $rest:tt )* ) => {
        literals!($name : $literal);
        literals!($( $rest )*);
    };

    ( pub $name:ident : $literal:expr , $( $rest:tt )* ) => {
        literals!(pub $name : $literal);
        literals!($( $rest )*);
    };

    ( @impl $name:ident : $literal:expr ) => {
        impl pegast::rules::literal::LiteralValue for $name {
            const LITERAL: &'static str = $literal;
        }
    };

    ( $(,)* ) => {};
}
