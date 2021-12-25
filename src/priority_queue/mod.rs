/// Pareto element trait. Defines an element that is present on the pareto front.
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

/// Defines a guided element. The element provides a guide function used by the priority queue.
pub trait GuidedElement<T:Ord> {
    /// returns the guide value of the element
    fn guide(&self) -> T;
}


/// Defines the behavior of a priority queue.
/// 
/// It allows to insert some guided element, remove the minimum or maximum, etc.
pub trait PriorityQueue<T,Elt> where Elt:GuidedElement<T>, T:Ord {

    /// peeks the minimum element in the queue
    fn peek_minimum(&self) -> Option<&Elt>;

    /// peeks the maximum element in the queue
    fn peek_maximum(&self) -> Option<&Elt>;

    /// pops the minimum element in the queue
    fn pop_minimum(&mut self) -> Option<Elt>;

    /// pops the maximum element in the queue
    fn pop_maximum(&mut self) -> Option<Elt>;

    /// inserts an element in the queue
    /// 
    /// returns true iff the element was successfully inserted.
    /// In some situations (for instance pareto priority queues, an insertion does not always
    /// leads to a successful insertion)
    fn insert(&mut self, elt:Elt) -> bool;
}


/// Implements pareto front specific functions
pub trait ParetoFront<T,Elt> where T:Ord, Elt:ParetoElement<T> {

    /// returns an element dominating the element if it exists
    fn find_dominating(&self, elt:&Elt) -> Option<&Elt>;
}


/// Pareto Priority-queue list.
/// 
/// Implements a Pareto priority queue. Each element is stored in a simple vector.
/// When inserting, check all other elements for dominations.
pub mod pareto_pq_list;

/// Utility class
pub mod util;