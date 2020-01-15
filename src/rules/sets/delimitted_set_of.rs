use {
    super::SetEntries,
    crate::{input::Input, ParseError, PegAstNode},
    std::{borrow::Cow, collections::HashMap},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelimittedSetOf<S: SetEntries, D: PegAstNode> {
    items: Vec<S>,
    delimitters: Vec<D>,
}

impl<S, D> DelimittedSetOf<S, D>
where
    S: SetEntries,
    D: PegAstNode,
{
    pub fn items(&self) -> impl Iterator<Item = &S> {
        self.items.iter()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut S> {
        self.items.iter_mut()
    }

    pub fn delimitters(&self) -> impl Iterator<Item = &D> {
        self.delimitters.iter()
    }

    pub fn delimitters_mut(&mut self) -> impl Iterator<Item = &mut D> {
        self.delimitters.iter_mut()
    }
}

impl<S, D> PegAstNode for DelimittedSetOf<S, D>
where
    S: SetEntries,
    D: PegAstNode,
{
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let mut items = Vec::new();
        let mut delimitters = Vec::new();
        let mut entry_counts: HashMap<S::EntryId, usize> = S::all_entry_ids()
            .into_iter()
            .map(|&entry_id| (entry_id, 0))
            .collect();

        let new_position = {
            let mut peek_input = input.peek_only();
            let mut position = peek_input.position();
            let mut ending_with_delimitter = false;

            while let Ok(item) = S::parse(&mut peek_input) {
                let entry_id = item.entry_id();
                let count = entry_counts
                    .get_mut(&entry_id)
                    .expect("Parsed an entry of a set that wasn't expected");

                if let Some(max_repetitions) = S::max_repetitions(entry_id) {
                    if *count >= max_repetitions {
                        break;
                    }
                }

                *count += 1;
                items.push(item);
                position = peek_input.position();
                ending_with_delimitter = false;

                if let Ok(delimitter) = D::parse(&mut peek_input) {
                    delimitters.push(delimitter);
                    ending_with_delimitter = true;
                } else {
                    break;
                }
            }

            for (entry_id, count) in entry_counts {
                if count < S::min_repetitions(entry_id) {
                    return Err(ParseError {
                        expected: Self::expecting(),
                        position: peek_input.position(),
                    });
                }
            }

            if ending_with_delimitter {
                delimitters.pop();
            }

            position
        };

        input.advance_to(new_position);

        Ok(DelimittedSetOf { items, delimitters })
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        let mut string = String::new();

        for (item, delimitter) in self.items.iter().zip(self.delimitters.iter()) {
            string.push_str(&item.parsed_string());
            string.push_str(&delimitter.parsed_string());
        }

        if let Some(last_item) = self.items.last() {
            string.push_str(&last_item.parsed_string());
        }

        Cow::Owned(string)
    }

    fn parsed_string_length(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.parsed_string_length())
            .chain(
                self.delimitters
                    .iter()
                    .map(|delimitter| delimitter.parsed_string_length()),
            )
            .sum()
    }

    fn expecting() -> Vec<String> {
        S::expecting()
    }
}
