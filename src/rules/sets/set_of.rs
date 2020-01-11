use {
    super::SetEntries,
    crate::{input::Input, ParseError, PegAstNode},
    std::{
        borrow::Cow,
        collections::HashMap,
        ops::{Deref, DerefMut},
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetOf<S: SetEntries>(Vec<S>);

impl<S> Deref for SetOf<S>
where
    S: SetEntries,
{
    type Target = Vec<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for SetOf<S>
where
    S: SetEntries,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S> PegAstNode for SetOf<S>
where
    S: SetEntries,
{
    fn parse(input: &mut impl Input) -> Result<Self, ParseError> {
        let mut parsed_nodes = Vec::new();
        let mut entry_counts: HashMap<S::EntryId, usize> = S::all_entry_ids()
            .into_iter()
            .map(|&entry_id| (entry_id, 0))
            .collect();

        let new_position = {
            let mut peek_input = input.peek_only();

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
                parsed_nodes.push(item);
            }

            for (entry_id, count) in entry_counts {
                if count < S::min_repetitions(entry_id) {
                    return Err(ParseError {
                        expected: Self::expecting(),
                        position: peek_input.position(),
                    });
                }
            }

            peek_input.position()
        };

        input.advance_to(new_position);

        Ok(SetOf(parsed_nodes))
    }

    fn parsed_string(&self) -> Cow<'_, str> {
        self.0.parsed_string()
    }

    fn parsed_string_length(&self) -> usize {
        self.0.parsed_string_length()
    }

    fn expecting() -> Vec<String> {
        S::expecting()
    }
}
