use {
    crate::{input::Input, ParseError, PegAstNode},
    std::borrow::Cow,
};

macro_rules! tuple_impl {
    (
        < $first_type:ident $( , $type:ident )* $(,)* >,
        ( $first_binding:ident $( , $binding:ident )* $(,)* ),
        ( $( $field:tt ),* $(,)* )
    ) => {
        impl<$first_type $(, $type)*> PegAstNode for ($first_type , $($type),*)
        where
            $first_type: PegAstNode,
            $( $type: PegAstNode, )*
        {
            fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
                let (new_position, tuple) = {
                    let mut peek_input = input.peek_only();
                    let $first_binding = $first_type::parse(&mut peek_input)?;
                    $( let $binding = $type::parse(&mut peek_input)?; )*

                    (peek_input.position(), ($first_binding, $( $binding ),*))
                };

                input.advance_to(new_position);

                Ok(tuple)
            }

            fn parsed_string(&self) -> Cow<'_, str> {
                let mut string = String::new();

                $( string.push_str(self.$field.parsed_string().as_ref()); )*

                Cow::Owned(string)
            }

            fn expecting() -> Vec<String> {
                $first_type::expecting()
            }
        }
    };
}

tuple_impl!(<A>, (a), (0));
tuple_impl!(<A, B>, (a, b), (0, 1));
tuple_impl!(<A, B, C>, (a, b, c), (0, 1, 2));
tuple_impl!(<A, B, C, D>, (a, b, c, d), (0, 1, 2, 3));
tuple_impl!(<A, B, C, D, E>, (a, b, c, d, e), (0, 1, 2, 3, 4));
