/// Pareto element trait. Defines an element present on the pareto front.
pub trait ParetoElement<T:Ord, const NB_DIM:usize> {
    /// returns the dimensions of the element
    fn coordinates(&self) -> &[T;NB_DIM];

    /// returns true iff the element dominates the other
    fn dominates(&self, other:&Self) -> bool;
}


/// Pareto front trait. Maintains pareto elements
/// Parametrized by:
///  - T: dimension values (usually integers or floating point numbers)
///  - Elt: Pareto elements
///  - ItQuery: Iterator over a query of the front
///  - It: Iterator over the elements of the front
///  - NB_DIM: number of dimensions
pub trait ParetoFront<'a, T, Elt, It, const NB_DIM:usize>
where T:Ord, Elt:'a+ParetoElement<T,NB_DIM>+Eq, It:Iterator<Item=&'a Elt> {

    /// Returns the list of elements intersecting the query (borders included)
    fn query(&'a self, min_bound:&[T;NB_DIM], max_bound:&[T;NB_DIM]) -> Vec<&'a Elt>;

    /// Returns and removes the element with minimum value on dimension `dim`
    fn pop_minimum_element(&mut self, dim:usize) -> Option<Elt>;

    /// Returns an element dominating the current one if it exists
    fn find_dominating(&self, elt:&Elt) -> Option<&Elt>;

    /// Inserts an element in the pareto front
    ///  - if the element is dominated, returns false
    ///  - if the element dominates some other elements, these elements are removed
    fn insert(&mut self, elt:Elt) -> bool;

    /// Removes an element from the pareto front.
    /// Returns true iff the element existed and was removed
    fn remove(&mut self, elt:&Elt) -> bool;

    /// iterator over the elements in the front
    fn iter(&'a self) -> It;
}

/// Simple Pareto front using a list to store labels.
///  - peek_min: O(1)
///  - insert: O(1)
///  - remove: O(n)
///  - find_dominated_by: O(n)
pub mod list;

/// naive kd-tree implementation.
/// maintains the pareto front as a tree-structure, in which each node stores a pareto element and
/// possibly children dividing the space
pub mod naive_kd_tree;

/// kd-tree implementation.
/// maintains the pareto front as a tree-structure, in which each node stores a pareto element and
/// possibly children dividing the space
pub mod kd_tree;

/// utility functions & data-structures
pub mod util;