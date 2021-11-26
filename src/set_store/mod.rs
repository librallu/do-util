/// Stores sets. Can perform insertions, remove, find subsets/supersets.
pub trait SetStore<T:Copy+Eq> {
    /// Iterator trait over subsets
    type SetIterator: Iterator<Item=Vec<T>>;

    /// Inserts a subset
    fn insert(&mut self, s:&[T]);

    /// Removes a subset.
    /// 
    /// returns true if the element existed
    fn remove(&mut self, s:&[T]) -> bool;

    /// enumerates all subsets
    fn find_subsets(&self, s:&[T]) -> Self::SetIterator;

    /// enumerates all supersets
    fn find_supersets(&self, s:&[T]) -> Self::SetIterator;

    /// returns true if the set exists in the store
    fn contains(&self, s:&[T]) -> bool;
}

/// stores sets as a list.
/// 
/// Linear complexities, but should be fast for a small number of sets.
pub mod list;