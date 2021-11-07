use std::{marker::PhantomData, mem::swap};

use crate::pareto_pq::{ParetoElement, ParetoFront};

type Link<Elt, const NB_DIM:usize> = Option<Box<Node<Elt,NB_DIM>>>;

/// node of the kd-tree.
#[derive(Debug)]
struct Node<Elt, const NB_DIM:usize> {
    /// element of the node
    e:Elt,
    /// left child
    l:Link<Elt,NB_DIM>,
    /// right child
    r:Link<Elt,NB_DIM>,
}

impl<Elt, const NB_DIM:usize> Node<Elt,NB_DIM> {
    /// returns the node element
    pub fn elt(&self) -> &Elt { &self.e }

    /// returns the left child
    pub fn left(&self) -> &Link<Elt,NB_DIM> { &self.l }

    /// returns the right child
    pub fn right(&self) -> &Link<Elt,NB_DIM> { &self.r }

    /// returns a mutable reference to left child
    pub fn left_mut(&mut self) -> &mut Link<Elt,NB_DIM> { &mut self.l }

    /// returns a mutable reference to right child
    pub fn right_mut(&mut self) -> &mut Link<Elt,NB_DIM> { &mut self.r }

    // /// returns true iff the node is a leaf
    // pub fn is_leaf(&self) -> bool { self.l.is_none() && self.r.is_none() }

    // /// detach the left child and return it
    // pub fn detach_left(&mut self) -> Link<Elt,NB_DIM> {
    //     let mut res = None;
    //     swap(&mut res, &mut self.l);
    //     res
    // }

    // /// detach the right child and return it
    // pub fn detach_right(&mut self) -> Link<Elt,NB_DIM> {
    //     let mut res = None;
    //     swap(&mut res, &mut self.r);
    //     res
    // }

    /// attch the sub-tree into the left child. Returns the previous left child
    pub fn attach_left(&mut self, t:Link<Elt,NB_DIM>) -> Link<Elt,NB_DIM> {
        let mut res = t;
        swap(&mut res, &mut self.l);
        res
    }

    /// attch the sub-tree into the right child. Returns the previous right child
    pub fn attach_right(&mut self, t:Link<Elt,NB_DIM>) -> Link<Elt,NB_DIM> {
        let mut res = t;
        swap(&mut res, &mut self.r);
        res
    }

    pub fn new(e:Elt, l:Link<Elt,NB_DIM>, r:Link<Elt,NB_DIM>) -> Self {
        Self { e, l, r }
    }

    /// decompose the node into (elt,left,right)
    pub fn decompose(self) -> (Elt, Link<Elt,NB_DIM>, Link<Elt,NB_DIM>) {
        (self.e, self.l, self.r)
    }
}



/// Kd-tree based pareto front structure
#[derive(Debug)]
pub struct NaiveKDTreeFront<T, Elt, const NB_DIM:usize> {
    /// root node
    root:Link<Elt,NB_DIM>,
    /// phantom for type T
    phantom_t:PhantomData<T>,
}

impl<T, Elt, const NB_DIM:usize> Default for NaiveKDTreeFront<T, Elt, NB_DIM> {
    fn default() -> Self {
        Self {
            root: None,
            phantom_t: PhantomData::default()
        }
    }
}

impl<T:Ord, Elt, const NB_DIM:usize> NaiveKDTreeFront<T, Elt, NB_DIM>
where Elt:ParetoElement<T,NB_DIM>+Eq+std::fmt::Debug, T:Copy {

    // /// pretty print of the tree structure (only for small trees)
    // fn pretty_print_node(node:&Node<Elt,NB_DIM>, prefix:&str) {
    //     println!("{}+ {:?}", prefix, node.elt());
    //     match node.left() {
    //         None => println!("{}|\tEmpty", prefix),
    //         Some(n) => Self::pretty_print_node(
    //             &*n,
    //             format!("{}|\t", prefix).as_str()
    //         )
    //     }
    //     match node.right() {
    //         None => println!("{}|\tEmpty", prefix),
    //         Some(n) => Self::pretty_print_node(
    //             &*n,
    //             format!("{}|\t", prefix).as_str()
    //         )
    //     }
    // }

    // /// pretty print the tree structure
    // fn pretty_print(&self) {
    //     match &self.root {
    //         Some(n) => Self::pretty_print_node(&*n, ""),
    //         None => println!("Empty tree"),
    //     }
    // }


    /// adds the element to a node without any dominance checks
    fn rec_insert_without_check(node:&mut Node<Elt,NB_DIM>, elt:Elt, dim:usize) {
        if elt.coordinates()[dim] < node.elt().coordinates()[dim] { // go left
            match node.left_mut() {
                None => { // insert here
                    node.attach_left(Some(Box::new(
                        Node::new(elt, None, None)
                    )));
                },
                Some(n) => {
                    Self::rec_insert_without_check(&mut *n, elt, (dim+1)%NB_DIM);
                }
            }
        } else { // go right
            match node.right_mut() {
                None => { // insert here
                    node.attach_right(Some(Box::new(
                        Node::new(elt, None, None)
                    )));
                },
                Some(n) => {
                    Self::rec_insert_without_check(&mut *n, elt, (dim+1)%NB_DIM);
                }
            }
        }
        
    }

    /// adds the element to a node without any dominance checks
    fn insert_without_check(&mut self, elt:Elt) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(elt, None, None)));
        } else {
            Self::rec_insert_without_check(self.root.as_mut().unwrap(), elt, 0);
        }
    }

    /// recursive search function. Returns the node with its decision dimension (depth % NB_DIM)
    fn rec_search<'a>(link:&'a mut Link<Elt,NB_DIM>, elt:&Elt, dim:usize)
    -> (&'a mut Link<Elt, NB_DIM>, Option<usize>) {
        if link.is_none() {
            (link, None)
        } else if link.as_ref().unwrap().elt() == elt {
            (link, Some(dim))
        } else {
            let next_link = if
            elt.coordinates()[dim] < link.as_ref().unwrap().elt().coordinates()[dim] { // go left
                link.as_mut().unwrap().left_mut()
            } else { // go right
                link.as_mut().unwrap().right_mut()
            };
            Self::rec_search(next_link, elt, (dim+1)%NB_DIM)
        }
    }

    /// search an element in the tree. Returns the node and the decision dimension (depth % NB_DIM)
    fn search(&mut self, elt:&Elt) -> (&mut Link<Elt, NB_DIM>, Option<usize>) {
        Self::rec_search(&mut self.root, elt, 0)
    }

    // fn search_minimum(&mut self, target_dim:usize) -> Option<&Elt> {
    //     Self::rec_search_minimum(&mut self.root, 0, target_dim).0.as_ref()
    //         .map(|n| n.elt())
    // }

    /// recursive function to search a minimum node given a dimension
    fn rec_search_minimum(link:&mut Link<Elt,NB_DIM>, dim:usize, target_dim:usize)
    -> (&mut Link<Elt, NB_DIM>, Option<usize>, Option<T>) {
        match link {
            None => (link, None, None),
            Some(node) => {
                let ev = node.elt().coordinates()[target_dim];
                if dim == target_dim {
                    if node.left().is_none() {
                        (link, Some(dim), Some(ev))
                    } else {
                        Self::rec_search_minimum(
                            link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                        )
                    }
                } else { // search left and right
                    let l_v = Self::rec_search_minimum(
                        link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                    ).2;
                    let r_v = Self::rec_search_minimum(
                        link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM, target_dim
                    ).2;
                    match (l_v, r_v) {
                        (None, None) => (link, Some(dim), Some(ev)),
                        (None, Some(rv)) => {
                            if rv < ev {
                                Self::rec_search_minimum(
                                    link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM, target_dim
                                )
                            } else { (link, Some(dim), Some(ev)) }
                        },
                        (Some(lv), None) => {
                            if lv < ev {
                                Self::rec_search_minimum(
                                    link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                                )
                            } else { (link, Some(dim), Some(ev)) }
                        },
                        (Some(lv), Some(rv)) => {
                            if lv < ev && lv < rv { // left
                                Self::rec_search_minimum(
                                    link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                                )
                            } else if rv < ev { // right
                                Self::rec_search_minimum(
                                    link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM, target_dim
                                )
                            } else { (link, Some(dim), Some(ev)) }
                        },
                    }
                }
            }
        }
    }

    /// removes a node in the tree
    fn remove_link(link: &mut Link<Elt,NB_DIM>, dim:usize) -> Option<Elt> {
        match link.take() {
            None => None, // link is empty, do nothing
            Some(mut node) => {
                match (node.left_mut().take(), node.right_mut().take()) {
                    (None, None) => { // node is a leaf, just decompose and return the element
                        let (res,_,_) = node.decompose();
                        Some(res)
                    },
                    // if right not null, search for minimum on current dimension
                    // then use it to replace (+ remove this "minimum" node).
                    (left , mut right @ Some(_)) => {
                        let (min_link, _, _) =
                            Self::rec_search_minimum(&mut right, dim, dim);
                        let mut elt = Self::remove_link(min_link, dim).unwrap();
                        swap(&mut elt, &mut node.e);
                        node.l = left;
                        node.r = right;
                        *link = Some(node);
                        Some(elt)
                    },
                    // if left not null, find minimum in the left subtree, replace + delete
                    // then put the left subtree to the right
                    (mut left @ Some(_), mut right @ None) => {
                        swap(&mut right, &mut left); // swap left and right subtrees
                        let (min_link, _, _) =
                            Self::rec_search_minimum(&mut right, dim, dim);
                        let mut elt = Self::remove_link(min_link, dim).unwrap();
                        swap(&mut elt, &mut node.e);
                        node.r = right;
                        *link = Some(node);
                        Some(elt)
                    }
                }
            }
        }
    }

    /// removes and return the element
    /// if the element does not exist, it returns None
    fn remove_and_return(&mut self, elt:&Elt) -> Option<Elt> {
        match self.search(elt) {
            (None, None) => None,
            (link, Some(dim)) => {
                Self::remove_link(link, dim)
            },
            _ => panic!("internal error")
        }
    }

    /// returns true if the front is empty
    pub fn is_empty(&self) -> bool { self.root.is_none() }

    /// recursive search for a dominating node
    fn rec_exists_dominating<'a>(link: &'a Link<Elt,NB_DIM>, elt:&Elt) -> Option<&'a Elt> {
        match link {
            None => None,
            Some(node) => {
                if node.elt().dominates(elt) { Some(node.elt()) }
                else {
                    match Self::rec_exists_dominating(node.left(), elt) {
                        Some(e) => Some(e),
                        None => { Self::rec_exists_dominating(node.right(), elt) }
                    }
                }
            }
        }
    }

    /// remove nodes having their dominated by the given element
    fn rec_remove_dominated_by(link: &mut Link<Elt,NB_DIM>, elt:&Elt, dim:usize) {
        if let Some(node) = link {
            Self::rec_remove_dominated_by(node.left_mut(), elt, (dim+1)%NB_DIM);
            Self::rec_remove_dominated_by(node.right_mut(), elt, (dim+1)%NB_DIM);
            if elt.dominates(node.elt()) {
                Self::remove_link(link, dim);
            }
        }
    }
}

impl<'a, T, Elt, const NB_DIM:usize> ParetoFront<'a,T,Elt,
    core::slice::Iter<'a,Elt>,
    NB_DIM
> for NaiveKDTreeFront<T, Elt, NB_DIM> where T:Ord+Copy, Elt:ParetoElement<T,NB_DIM>+Eq+std::fmt::Debug {
    fn query(&'a self, _min_bound:&[T;NB_DIM], _max_bound:&[T;NB_DIM]) -> Vec<&'a Elt> {
        todo!()
    }

    fn pop_minimum_element(&mut self, dim:usize) -> Option<Elt> {
        let (min_node,min_dim,_) = Self::rec_search_minimum(
            &mut self.root, 0, dim
        );
        match min_dim {
            Some(d) => {
                Self::remove_link(min_node, d)
            },
            None => None,
        }
    }

    fn find_dominating(&self, elt:&Elt) -> Option<&Elt> {
        Self::rec_exists_dominating(&self.root, elt)
    }

    fn insert(&mut self, elt:Elt) -> bool {
        if self.find_dominating(&elt).is_some() { return false; } // dominated, stop here
        // find all elements dominated by elt
        Self::rec_remove_dominated_by(&mut self.root, &elt, 0);
        // finally insert the element
        self.insert_without_check(elt);
        true
    }

    fn remove(&mut self, elt:&Elt) -> bool {
        self.remove_and_return(elt).is_some()
    }

    fn iter(&'a self) -> core::slice::Iter<'a,Elt> {
        todo!()
    }
}


#[cfg(test)]
pub mod test {
    use crate::pareto_pq::util::CartesianParetoElement;

    use super::*;

    #[test]
    pub fn test_simple() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([0,1]));
        front.insert_without_check(CartesianParetoElement::new([1,0]));
        front.insert_without_check(CartesianParetoElement::new([2,0]));
        front.insert_without_check(CartesianParetoElement::new([0,0]));
        // println!("{:?}", front);
        // front.pretty_print();
        assert!(front.search(&CartesianParetoElement::new([0,1])).0.is_some());
        assert!(front.search(&CartesianParetoElement::new([1,0])).0.is_some());
        assert!(front.search(&CartesianParetoElement::new([2,0])).0.is_some());
        assert!(front.search(&CartesianParetoElement::new([1,1])).0.is_none());
        assert!(front.search(&CartesianParetoElement::new([0,0])).0.is_some());
    }

    #[test]
    pub fn test_find_min_0() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([0,10]));
        front.insert_without_check(CartesianParetoElement::new([10,5]));
        front.insert_without_check(CartesianParetoElement::new([20,0]));
        // front.pretty_print();
        // assert_eq!(front.search_minimum(0).unwrap(), &CartesianParetoElement::new([0,10]));
    }
    
    #[test]
    pub fn test_find_min_1() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([0,10]));
        front.insert_without_check(CartesianParetoElement::new([10,5]));
        front.insert_without_check(CartesianParetoElement::new([20,0]));
        // front.pretty_print();
        // assert_eq!(front.search_minimum(1).unwrap(), &CartesianParetoElement::new([20,0]));
    }

    #[test]
    pub fn test_remove_empty() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        assert!(front.remove_and_return(&CartesianParetoElement::new([0,0])).is_none());
    }


    #[test]
    pub fn test_remove_1() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_some());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_remove_2() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        assert!(front.remove_and_return(&CartesianParetoElement::new([0,0])).is_none());
        assert!(!front.is_empty());
    }

    #[test]
    pub fn test_remove_3() { // delete center, right, left
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        front.insert_without_check(CartesianParetoElement::new([5,5]));
        front.insert_without_check(CartesianParetoElement::new([20,20]));
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_none());
        assert!(!front.is_empty());
        assert!(front.search(&CartesianParetoElement::new([20,20])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_none());
        assert!(!front.is_empty());
        // front.pretty_print();
        assert!(front.search(&CartesianParetoElement::new([5,5])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_none());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_remove_4() { // delete right, center, left
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        front.insert_without_check(CartesianParetoElement::new([5,5]));
        front.insert_without_check(CartesianParetoElement::new([20,20]));
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_none());
        assert!(!front.is_empty());
        assert!(front.search(&CartesianParetoElement::new([10,10])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_none());
        assert!(!front.is_empty());
        // front.pretty_print();
        assert!(front.search(&CartesianParetoElement::new([5,5])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_none());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_remove_5() { // delete left, center, right
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        front.insert_without_check(CartesianParetoElement::new([5,5]));
        front.insert_without_check(CartesianParetoElement::new([20,20]));
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([5,5])).is_none());
        assert!(!front.is_empty());
        assert!(front.search(&CartesianParetoElement::new([10,10])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_none());
        assert!(!front.is_empty());
        // front.pretty_print();
        assert!(front.search(&CartesianParetoElement::new([20,20])).0.is_some());
        // front.pretty_print();
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_some());
        assert!(front.remove_and_return(&CartesianParetoElement::new([20,20])).is_none());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_pop_min_1() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        front.insert_without_check(CartesianParetoElement::new([5,5]));
        front.insert_without_check(CartesianParetoElement::new([20,20]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,5]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([10,10]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([20,20]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_no_domination() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        front.insert(CartesianParetoElement::new([5,10]));
        front.insert(CartesianParetoElement::new([10,5]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,10]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([10,5]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_existing() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert!(!front.insert(CartesianParetoElement::new([10,10])));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_inserted() {
        let mut front:NaiveKDTreeFront<u16, CartesianParetoElement<2>, 2> = NaiveKDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([10,10])));
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

}