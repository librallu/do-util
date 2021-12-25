/// Pareto element trait. Defines an element present on the pareto front.
pub trait ParetoElement<T:Ord> {
    /// Iterator trait over the coordinates of the element
    type CoordIterator: Iterator<Item=T>;

    /// returns the dimensions of the element
    /// the return value has to match the number of dimensions of the element
    fn coordinates(&self) -> Self::CoordIterator;

    /// returns true iff the element dominates the other
    fn dominates(&self, other:&Self) -> bool;

    /// returns the number of dimensions
    fn nb_dimensions(&self) -> usize;

    /// returns the k-th coordinate
    fn kth(&self, k:usize) -> T;
}


/// Pareto front trait. Maintains pareto elements
/// Parametrized by:
///  - T: dimension values (usually integers or floating point numbers)
///  - Elt: Pareto elements
///  - ItQuery: Iterator over a query of the front
///  - It: Iterator over the elements of the front
///  - NB_DIM: number of dimensions
pub trait ParetoFront<T, Elt>
where T:Ord, Elt:ParetoElement<T>+Eq {

    /// Returns the list of elements intersecting the query (borders included)
    fn query(&self, min_bound:&[T], max_bound:&[T]) -> Vec<&Elt>;

    /// Returns and removes the element with minimum value on dimension `dim`
    fn pop_minimum_element(&mut self, dim:usize) -> Option<Elt>;

    /// Returns the element with minimum value on dimension `dim`
    fn peek_minimum_element(&self, dim:usize) -> Option<&Elt>;

    /// Returns the minimum value on coordinate `dim`
    fn peek_minimum_coordinate(&self, dim:usize) -> Option<T> {
        self.peek_minimum_element(dim).map(|e| e.kth(dim))
    }

    /// Returns an element dominating the current one if it exists
    fn find_dominating(&self, elt:&Elt) -> Option<&Elt>;

    /// Inserts an element in the pareto front
    ///  - if the element is dominated, returns false, otherwise, returns true.
    ///  - if the element dominates some other elements, these elements are removed
    fn insert(&mut self, elt:Elt) -> bool;

    /// Removes an element from the pareto front.
    /// Returns true iff the element existed and was removed
    fn remove(&mut self, elt:&Elt) -> bool;

    /// creates a new empty instance
    /// discretization_hint: slice of (LB,UB,step)
    fn create_empty(discretization_hint:&[Option<(T,T,T)>]) -> Self;
}

/// utility functions & data-structures
pub mod util;

/// Simple Pareto front using a list to store labels.
///  - peek_min: O(1)
///  - insert: O(1)
///  - remove: O(n)
///  - find_dominated_by: O(n)
pub mod list;

/// kd-tree implementation.
/// maintains the pareto front as a tree-structure, in which each node stores a pareto element and
/// possibly children dividing the space
pub mod kd_tree;
