use {
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, iter},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delimitted<T, D> {
    pub head: T,
    pub tail: Vec<(D, T)>,
}

impl<T, D> Delimitted<T, D> {
    pub fn items(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.head).chain(self.tail.iter().map(|(_, item)| item))
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut T> {
        iter::once(&mut self.head).chain(self.tail.iter_mut().map(|(_, item)| item))
    }

    pub fn delimitters(&self) -> impl Iterator<Item = &D> {
        self.tail.iter().map(|(delimitter, _)| delimitter)
    }

    pub fn delimitters_mut(&mut self) -> impl Iterator<Item = &mut D> {
        self.tail.iter_mut().map(|(delimitter, _)| delimitter)
    }

    pub fn items_and_delimitters(&self) -> impl Iterator<Item = ItemOrDelimitter<&'_ T, &'_ D>> {
        iter::once(ItemOrDelimitter::Item(&self.head)).chain(self.tail.iter().flat_map(
            |(delimitter, item)| {
                iter::once(ItemOrDelimitter::Delimitter(delimitter))
                    .chain(iter::once(ItemOrDelimitter::Item(item)))
            },
        ))
    }

    pub fn items_and_delimitters_mut(
        &mut self,
    ) -> impl Iterator<Item = ItemOrDelimitter<&'_ mut T, &'_ mut D>> {
        iter::once(ItemOrDelimitter::Item(&mut self.head)).chain(self.tail.iter_mut().flat_map(
            |(delimitter, item)| {
                iter::once(ItemOrDelimitter::Delimitter(delimitter))
                    .chain(iter::once(ItemOrDelimitter::Item(item)))
            },
        ))
    }
}

impl<T, D> PegAstNode for Delimitted<T, D>
where
    T: PegAstNode,
    D: PegAstNode,
{
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let (new_position, result) = {
            let mut peek_input = input.peek_only();
            let head = T::parse(&mut peek_input)?;
            let tail = Vec::parse(&mut peek_input)?;

            (peek_input.position(), Delimitted { head, tail })
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemOrDelimitter<T, D> {
    Item(T),
    Delimitter(D),
}
