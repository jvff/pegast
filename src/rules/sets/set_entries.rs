use {crate::PegAstNode, std::hash::Hash};

pub trait SetEntries: PegAstNode {
    type EntryId: Clone + Copy + Eq + Hash + PartialEq + 'static;

    fn all_entry_ids() -> &'static [Self::EntryId];
    fn entry_id(&self) -> Self::EntryId;
    fn min_repetitions(entry_id: Self::EntryId) -> usize;
    fn max_repetitions(entry_id: Self::EntryId) -> Option<usize>;
}
