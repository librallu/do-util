use std::{marker::PhantomData, mem::swap};

use crate::pareto_pq::{ParetoElement, ParetoFront};

type Link<T, Elt, const NB_DIM:usize> = Option<Box<Node<T, Elt,NB_DIM>>>;

/// node of the kd-tree.
#[derive(Debug)]
struct Node<T, Elt, const NB_DIM:usize> {
    /// element of the node
    e:Elt,
    /// left child
    l:Link<T, Elt,NB_DIM>,
    /// right child
    r:Link<T, Elt,NB_DIM>,
    /// lower and upper bounds on dimensions
    b:[(T,T);NB_DIM]
}

impl<T, Elt, const NB_DIM:usize> Node<T, Elt,NB_DIM>
where T:Ord+Copy, Elt:ParetoElement<T>{
    /// returns the node element
    pub fn elt(&self) -> &Elt { &self.e }

    /// returns the left child
    pub fn left(&self) -> &Link<T, Elt,NB_DIM> { &self.l }

    /// returns the right child
    pub fn right(&self) -> &Link<T, Elt,NB_DIM> { &self.r }

    /// returns a mutable reference to left child
    pub fn left_mut(&mut self) -> &mut Link<T, Elt,NB_DIM> { &mut self.l }

    /// returns a mutable reference to right child
    pub fn right_mut(&mut self) -> &mut Link<T, Elt,NB_DIM> { &mut self.r }

    /// returns the bounds of the link
    pub fn bounds(&self) -> &[(T,T);NB_DIM] { &self.b }

    /// attch the sub-tree into the left child. Returns the previous left child
    pub fn attach_left(&mut self, t:Link<T,Elt,NB_DIM>) -> Link<T,Elt,NB_DIM> {
        let mut res = t;
        swap(&mut res, &mut self.l);
        self.b = Self::compute_bounds(&self.e, &self.l, &self.r);
        res
    }

    /// attch the sub-tree into the right child. Returns the previous right child
    pub fn attach_right(&mut self, t:Link<T,Elt,NB_DIM>) -> Link<T,Elt,NB_DIM> {
        let mut res = t;
        swap(&mut res, &mut self.r);
        self.b = Self::compute_bounds(&self.e, &self.l, &self.r);
        res
    }

    pub fn new(e:Elt, l:Link<T,Elt,NB_DIM>, r:Link<T,Elt,NB_DIM>) -> Self {
        let b = Self::compute_bounds(&e, &l, &r);
        Self { e, l, r, b }
    }

    /// decompose the node into (elt,left,right)
    pub fn decompose(self) -> (Elt, Link<T,Elt,NB_DIM>, Link<T,Elt,NB_DIM>) {
        (self.e, self.l, self.r)
    }

    /// update bounds of the node
    pub fn update_bounds(&mut self) {
        self.b = Self::compute_bounds(&self.e, &self.l, &self.r);
    }

    /// compute the bounds given e, left, right
    pub fn compute_bounds(e:&Elt, l:&Link<T,Elt,NB_DIM>, r:&Link<T,Elt,NB_DIM>) -> [(T,T);NB_DIM] {
        let dummy_t = e.kth(0);
        let mut res:[(T,T);NB_DIM] = [(dummy_t,dummy_t);NB_DIM];
        for (i,(a,b)) in e.coordinates().zip(e.coordinates())
        .map(|(a,b)| (a,b)).enumerate() {
            res[i] = (a,b);
        }
        if let Some(n) = l {
            for (i,(lower,upper)) in n.bounds().iter().enumerate() {
                res[i] = (
                    std::cmp::min(res[i].0, *lower),
                    std::cmp::max(res[i].1, *upper),
                );
            }
        }
        if let Some(n) = r {
            for (i,(lower,upper)) in n.bounds().iter().enumerate() {
                res[i] = (
                    std::cmp::min(res[i].0, *lower),
                    std::cmp::max(res[i].1, *upper),
                );
            }
        }
        res
    }
}



/// Kd-tree based pareto front structure
#[derive(Debug)]
pub struct KDTreeFront<T, Elt, const NB_DIM:usize> {
    /// root node
    root:Link<T,Elt,NB_DIM>,
    /// phantom for type T
    phantom_t:PhantomData<T>,
}

impl<T, Elt, const NB_DIM:usize> Default for KDTreeFront<T, Elt, NB_DIM> {
    fn default() -> Self {
        Self {
            root: None,
            phantom_t: PhantomData::default()
        }
    }
}

impl<T:Ord, Elt, const NB_DIM:usize> KDTreeFront<T, Elt, NB_DIM>
where Elt:ParetoElement<T>+Eq+std::fmt::Debug, T:Copy+std::fmt::Debug {

    /// adds the element to a node without any dominance checks
    fn rec_insert_without_check(node:&mut Node<T,Elt,NB_DIM>, elt:Elt, dim:usize) {
        if elt.kth(dim) < node.elt().kth(dim) { // go left
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
        node.update_bounds();
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
    fn rec_search<'a>(link:&'a mut Link<T,Elt,NB_DIM>, elt:&Elt, dim:usize)
    -> (&'a mut Link<T,Elt,NB_DIM>, Option<usize>) {
        if link.is_none() {
            (link, None)
        } else if link.as_ref().unwrap().elt() == elt {
            (link, Some(dim))
        } else {
            let next_link = if
            elt.kth(dim) < link.as_ref().unwrap().elt().kth(dim) { // go left
                link.as_mut().unwrap().left_mut()
            } else { // go right
                link.as_mut().unwrap().right_mut()
            };
            Self::rec_search(next_link, elt, (dim+1)%NB_DIM)
        }
    }

    /// search an element in the tree. Returns the node and the decision dimension (depth % NB_DIM)
    fn search(&mut self, elt:&Elt) -> (&mut Link<T,Elt,NB_DIM>, Option<usize>) {
        Self::rec_search(&mut self.root, elt, 0)
    }

    /// mutable recursive function to search a minimum node given a dimension
    fn mut_rec_search_minimum(link:&mut Link<T,Elt,NB_DIM>, dim:usize, target_dim:usize)
    -> (&mut Link<T,Elt,NB_DIM>, Option<usize>, Option<T>) {
        match link {
            None => (link, None, None),
            Some(node) => {
                // identify the direction to search
                let v_e = node.elt().kth(target_dim);
                let v_l = node.left().as_ref().map(|n| n.bounds()[target_dim].0);
                let v_r = node.right().as_ref().map(|n| n.bounds()[target_dim].0);
                match (v_l, v_r) {
                    (None, None) => (link, Some(dim), Some(v_e)),
                    (None, Some(vr)) => {
                        if vr < v_e {
                            Self::mut_rec_search_minimum(
                                link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                    (Some(vl), None) => {
                        if vl < v_e {
                            Self::mut_rec_search_minimum(
                                link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                    (Some(vl), Some(vr)) => {
                        if vl < v_e && vl < vr {
                            Self::mut_rec_search_minimum(
                                link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM, target_dim
                            )
                        } else if vr < v_e {
                            Self::mut_rec_search_minimum(
                                link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                }
            }
        }
    }

    /// recursive function to search a minimum node given a dimension
    fn rec_search_minimum(link:&Link<T,Elt,NB_DIM>, dim:usize, target_dim:usize)
    -> (&Link<T,Elt,NB_DIM>, Option<usize>, Option<T>) {
        match link {
            None => (link, None, None),
            Some(node) => {
                // identify the direction to search
                let v_e = node.elt().kth(target_dim);
                let v_l = node.left().as_ref().map(|n| n.bounds()[target_dim].0);
                let v_r = node.right().as_ref().map(|n| n.bounds()[target_dim].0);
                match (v_l, v_r) {
                    (None, None) => (link, Some(dim), Some(v_e)),
                    (None, Some(vr)) => {
                        if vr < v_e {
                            Self::rec_search_minimum(
                                link.as_ref().unwrap().right(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                    (Some(vl), None) => {
                        if vl < v_e {
                            Self::rec_search_minimum(
                                link.as_ref().unwrap().left(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                    (Some(vl), Some(vr)) => {
                        if vl < v_e && vl < vr {
                            Self::rec_search_minimum(
                                link.as_ref().unwrap().left(), (dim+1)%NB_DIM, target_dim
                            )
                        } else if vr < v_e {
                            Self::rec_search_minimum(
                                link.as_ref().unwrap().right(), (dim+1)%NB_DIM, target_dim
                            )
                        } else { (link, Some(dim), Some(v_e)) }
                    },
                }
            }
        }
    }

    /// removes a node in the tree
    fn remove_link(link: &mut Link<T,Elt,NB_DIM>, dim:usize) -> Option<Elt> {
        match link.take() {
            None => None, // link is empty, do nothing
            Some(mut node) => {
                let res = match (node.left_mut().take(), node.right_mut().take()) {
                    (None, None) => { // node is a leaf, just decompose and return the element
                        let (res,_,_) = node.decompose();
                        Some(res)
                    },
                    // if right not null, search for minimum on current dimension
                    // then use it to replace (+ remove this "minimum" node).
                    (left , mut right @ Some(_)) => {
                        let (min_link, _, _) =
                            Self::mut_rec_search_minimum(&mut right, dim, dim);
                        let mut elt = Self::remove_link(min_link, dim).unwrap();
                        swap(&mut elt, &mut node.e);
                        node.l = left;
                        node.r = right;
                        node.update_bounds();
                        *link = Some(node);
                        Some(elt)
                    },
                    // if left not null, find minimum in the left subtree, replace + delete
                    // then put the left subtree to the right
                    (mut left @ Some(_), mut right @ None) => {
                        swap(&mut right, &mut left); // swap left and right subtrees
                        let (min_link, _, _) =
                            Self::mut_rec_search_minimum(&mut right, dim, dim);
                        let mut elt = Self::remove_link(min_link, dim).unwrap();
                        swap(&mut elt, &mut node.e);
                        node.r = right;
                        node.update_bounds();
                        *link = Some(node);
                        Some(elt)
                    }
                };
                res
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

    /// recursive search for a dominating node. Returns a node dominating the element if it exists
    fn rec_exists_dominating<'a>(link: &'a Link<T,Elt,NB_DIM>, elt:&Elt) -> Option<&'a Elt> {
        match link {
            None => None,
            Some(node) => {
                // if the element has a coordinate lower than the bound, return None
                for (i,d) in elt.coordinates().enumerate() {
                    if d < node.bounds()[i].0 { return None; }
                }
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

    /// remove elements dominated by the given element
    fn rec_remove_dominated_by(link: &mut Link<T,Elt,NB_DIM>, elt:&Elt, dim:usize) {
        if let Some(node) = link {
            // if the element has a coordinate larger than the bound, return None
            for (i,d) in elt.coordinates().enumerate() {
                if d > node.bounds()[i].1 { return; }
            }
            Self::rec_remove_dominated_by(node.left_mut(), elt, (dim+1)%NB_DIM);
            Self::rec_remove_dominated_by(node.right_mut(), elt, (dim+1)%NB_DIM);
            if elt.dominates(node.elt()) {
                Self::remove_link(link, dim);
            }
        }
    }
}


impl<T, Elt, const NB_DIM:usize> ParetoFront<T,Elt>
for KDTreeFront<T,Elt,NB_DIM>
where T:Ord+Copy+std::fmt::Debug, Elt:ParetoElement<T>+Eq+std::fmt::Debug {

    fn query(&self, _min_bound:&[T], _max_bound:&[T]) -> Vec<&Elt> {
        todo!()
    }

    fn pop_minimum_element(&mut self, dim:usize) -> Option<Elt> {
        let (min_node,min_dim,_) = Self::mut_rec_search_minimum(
            &mut self.root, 0, dim
        );
        match min_dim {
            Some(d) => {
                Self::remove_link(min_node, d)
            },
            None => None,
        }
    }

    fn peek_minimum_element(&self, dim:usize) -> Option<&Elt> {
        let (min_node,_,_) = Self::rec_search_minimum(
            &self.root, 0, dim
        );
        min_node.as_ref().map(|e| e.elt())
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

    fn create_empty() -> Self {
        Self::default()
    }
}


#[cfg(test)]
pub mod test {
    use crate::pareto_pq::util::CartesianParetoElement;

    use super::*;

    #[test]
    pub fn test_simple() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
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
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([0,10]));
        front.insert_without_check(CartesianParetoElement::new([10,5]));
        front.insert_without_check(CartesianParetoElement::new([20,0]));
        // front.pretty_print();
        // assert_eq!(front.search_minimum(0).unwrap(), &CartesianParetoElement::new([0,10]));
    }
    
    #[test]
    pub fn test_find_min_1() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([0,10]));
        front.insert_without_check(CartesianParetoElement::new([10,5]));
        front.insert_without_check(CartesianParetoElement::new([20,0]));
        // front.pretty_print();
        // assert_eq!(front.search_minimum(1).unwrap(), &CartesianParetoElement::new([20,0]));
    }

    #[test]
    pub fn test_remove_empty() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.remove_and_return(&CartesianParetoElement::new([0,0])).is_none());
    }


    #[test]
    pub fn test_remove_1() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        assert!(front.remove_and_return(&CartesianParetoElement::new([10,10])).is_some());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_remove_2() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert_without_check(CartesianParetoElement::new([10,10]));
        assert!(front.remove_and_return(&CartesianParetoElement::new([0,0])).is_none());
        assert!(!front.is_empty());
    }

    #[test]
    pub fn test_remove_3() { // delete center, right, left
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
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
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
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
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
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
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
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
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert(CartesianParetoElement::new([5,10]));
        front.insert(CartesianParetoElement::new([10,5]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,10]));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([10,5]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_existing() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert!(!front.insert(CartesianParetoElement::new([10,10])));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_inserted() {
        let mut front:KDTreeFront<u16, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([10,10])));
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert_eq!(front.pop_minimum_element(0).unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

}