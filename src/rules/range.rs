use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{
        borrow::Cow,
        convert::TryFrom,
        error::Error,
        fmt::{self, Debug, Display, Formatter},
        marker::PhantomData,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfRangeError<V: RangeValue>(char, PhantomData<V>);

impl<V: RangeValue> Display for OutOfRangeError<V> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "input character '{}' is out of the range [{}-{}]",
            self.0,
            V::START,
            V::END
        )
    }
}

impl<V: RangeValue> Error for OutOfRangeError<V> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range<V: RangeValue>(char, PhantomData<V>);

impl<V: RangeValue> TryFrom<char> for Range<V> {
    type Error = OutOfRangeError<V>;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if character >= V::START && character <= V::END {
            Ok(Range(character, PhantomData))
        } else {
            Err(OutOfRangeError(character, PhantomData))
        }
    }
}

impl<V: RangeValue> PegAstNode for Range<V> {
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        if let Some(input_char) = input.peek() {
            if let Ok(result) = Self::try_from(input_char) {
                let _ = input.next();
                return Ok(result);
            }
        }

        Err(ParseError {
            expected: Self::expecting(),
            position: input.position(),
        })
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        Cow::Owned(self.0.to_string())
    }

    fn expecting() -> Vec<String> {
        vec![format!(
            "a character between '{}' and '{}'",
            V::START,
            V::END
        )]
    }
}

pub trait RangeValue: Debug {
    const START: char;
    const END: char;
}

#[macro_export]
macro_rules! ranges {
    ( $name:ident : [ $start:expr , $end:expr ] ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {}

        ranges!(@impl $name : [ $start, $end ]);
    };

    ( pub $name:ident : [ $start:expr , $end:expr ] ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {}

        ranges!(@impl $name : [ $start, $end ]);
    };

    ( $name:ident : [ $start:expr , $end:expr ] , $( $rest:tt )* ) => {
        ranges!($name : [ $start, $end ]);
        ranges!($( $rest )*);
    };

    ( pub $name:ident : [ $start:expr , $end:expr ] , $( $rest:tt )* ) => {
        ranges!(pub $name : [ $start, $end ]);
        ranges!($( $rest )*);
    };

    ( @impl $name:ident : [ $start:expr , $end:expr ] ) => {
        impl pegast::rules::range::RangeValue for $name {
            const START: char = $start;
            const END: char = $end;
        }
    };

    ( $(,)* ) => {};
}
