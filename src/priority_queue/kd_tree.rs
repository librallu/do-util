use std::{marker::PhantomData, mem::swap};

use crate::priority_queue::{GuidedElement, ParetoElement, PriorityQueue, ParetoFront};

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
    b:[(T,T);NB_DIM],
    /// guide lower bound
    guide_lb:T,
    /// guide upper bound
    guide_ub:T,
}

impl<T, Elt, const NB_DIM:usize> Node<T, Elt,NB_DIM>
where T:Ord+Copy, Elt:ParetoElement<T>+GuidedElement<T> {
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
        let (b, lb, ub) = Self::compute_bounds(&self.e, &self.l, &self.r);
        self.b = b;
        self.guide_lb = lb;
        self.guide_ub = ub;
        res
    }

    /// attch the sub-tree into the right child. Returns the previous right child
    pub fn attach_right(&mut self, t:Link<T,Elt,NB_DIM>) -> Link<T,Elt,NB_DIM> {
        let mut res = t;
        swap(&mut res, &mut self.r);
        let (b, lb, ub) = Self::compute_bounds(&self.e, &self.l, &self.r);
        self.b = b;
        self.guide_lb = lb;
        self.guide_ub = ub;
        res
    }

    pub fn new(e:Elt, l:Link<T,Elt,NB_DIM>, r:Link<T,Elt,NB_DIM>) -> Self {
        let (b, lb, ub) = Self::compute_bounds(&e, &l, &r);
        Self { e, l, r, b, guide_lb:lb, guide_ub:ub }
    }

    /// decompose the node into (elt,left,right)
    pub fn decompose(self) -> (Elt, Link<T,Elt,NB_DIM>, Link<T,Elt,NB_DIM>) {
        (self.e, self.l, self.r)
    }

    /// update bounds of the node
    pub fn update_bounds(&mut self) {
        let (b,lb,ub) = Self::compute_bounds(&self.e, &self.l, &self.r);
        self.b = b;
        self.guide_lb = lb;
        self.guide_ub = ub;
    }

    /// compute the bounds given e, left, right
    pub fn compute_bounds(e:&Elt, l:&Link<T,Elt,NB_DIM>, r:&Link<T,Elt,NB_DIM>) -> ([(T,T);NB_DIM],T,T) {
        let dummy_t = e.kth(0);
        let mut res:[(T,T);NB_DIM] = [(dummy_t,dummy_t);NB_DIM];
        for (i,(a,b)) in e.coordinates().zip(e.coordinates())
        .map(|(a,b)| (a,b)).enumerate() {
            res[i] = (a,b);
        }
        let mut lb = e.guide();
        let mut ub = e.guide();
        if let Some(n) = l {
            for (i,(lower,upper)) in n.bounds().iter().enumerate() {
                res[i] = (
                    std::cmp::min(res[i].0, *lower),
                    std::cmp::max(res[i].1, *upper),
                );
                lb = std::cmp::min(n.guide_lb, lb);
                ub = std::cmp::max(n.guide_ub, ub);
            }
        }
        if let Some(n) = r {
            for (i,(lower,upper)) in n.bounds().iter().enumerate() {
                res[i] = (
                    std::cmp::min(res[i].0, *lower),
                    std::cmp::max(res[i].1, *upper),
                );
                lb = std::cmp::min(n.guide_lb, lb);
                ub = std::cmp::max(n.guide_ub, ub);
            }
        }
        
        (res, lb, ub)
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
        Self { root: None, phantom_t: PhantomData::default() }
    }
}

impl<T, Elt, const NB_DIM:usize> PriorityQueue<T, Elt> for KDTreeFront<T, Elt, NB_DIM>
where T:Ord+Copy, Elt:GuidedElement<T>+ParetoElement<T> {
    fn peek_minimum(&self) -> Option<&Elt> {
        let (link,_) = Self::rec_search_min_guide(&self.root, 0);
        link.as_ref().map(|n| n.elt())
    }

    fn peek_maximum(&self) -> Option<&Elt> {
        todo!()
    }

    fn pop_minimum(&mut self) -> Option<Elt> {
        let (link,dim) = Self::mut_rec_search_min_guide(&mut self.root, 0);
        match dim {
            Some(d) => {
                let res = Self::remove_link(link, d);
                Self::rec_update_bounds(&mut self.root);
                res
            },
            None => None,
        }
    }

    fn pop_maximum(&mut self) -> Option<Elt> {
        todo!()
    }

    fn insert(&mut self, elt:Elt) -> bool {
        if self.find_dominating(&elt).is_some() { return false; } // dominated, stop here
        // find all elements dominated by elt
        Self::rec_remove_dominated_by(&mut self.root, &elt, 0);
        // finally insert the element
        self.insert_without_check(elt);
        true
    }

    fn peek_minimum_guide(&self) -> Option<T> {
        self.root.as_ref().map(|node| node.guide_lb)
    }

    fn peek_maximum_guide(&self) -> Option<T> {
        self.root.as_ref().map(|node| node.guide_ub)
    }
}

impl<T, Elt, const NB_DIM:usize> ParetoFront<T, Elt> for KDTreeFront<T, Elt, NB_DIM>
where T:Ord+Copy, Elt:GuidedElement<T>+ParetoElement<T> {
    fn find_dominating(&self, elt:&Elt) -> Option<&Elt> {
        Self::rec_exists_dominating(&self.root, elt)
    }
}

impl<T, Elt, const NB_DIM:usize> KDTreeFront<T, Elt, NB_DIM>
where T:Ord+Copy, Elt:GuidedElement<T>+ParetoElement<T> {

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

    /// finds the link with the minimum guide
    /// returns the link, with the current dimension
    fn rec_search_min_guide(link:&Link<T,Elt,NB_DIM>, dim:usize) -> (&Link<T,Elt,NB_DIM>, Option<usize>) {
        match link {
            None => (link, None),
            Some(node) => {
                let ge = node.elt().guide();
                let g_l = node.left().as_ref().map(|n| n.guide_lb);
                let g_r = node.right().as_ref().map(|n| n.guide_lb);
                match (g_l,g_r) {
                    (None,None) => (link, Some(dim)),
                    (None,Some(gr)) => {
                        if gr < ge {
                            Self::rec_search_min_guide(
                                link.as_ref().unwrap().right(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    },
                    (Some(gl),None) => {
                        if gl < ge {
                            Self::rec_search_min_guide(
                                link.as_ref().unwrap().left(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    }, 
                    (Some(gl),Some(gr)) => {
                        if gl < ge && gl < gr {
                            Self::rec_search_min_guide(
                                link.as_ref().unwrap().left(), (dim+1)%NB_DIM
                            )
                        } else if gr < ge {
                            Self::rec_search_min_guide(
                                link.as_ref().unwrap().right(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    }
                }
            }
        }
    }

    /// finds the link with the minimum guide
    /// returns the link, with the current dimension
    fn mut_rec_search_min_guide(link:&mut Link<T,Elt,NB_DIM>, dim:usize) -> (&mut Link<T,Elt,NB_DIM>, Option<usize>) {
        match link {
            None => (link, None),
            Some(node) => {
                let ge = node.elt().guide();
                let g_l = node.left().as_ref().map(|n| n.guide_lb);
                let g_r = node.right().as_ref().map(|n| n.guide_lb);
                match (g_l,g_r) {
                    (None,None) => (link, Some(dim)),
                    (None,Some(gr)) => {
                        if gr < ge {
                            Self::mut_rec_search_min_guide(
                                link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    },
                    (Some(gl),None) => {
                        if gl < ge {
                            Self::mut_rec_search_min_guide(
                                link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    }, 
                    (Some(gl),Some(gr)) => {
                        if gl < ge && gl < gr {
                            Self::mut_rec_search_min_guide(
                                link.as_mut().unwrap().left_mut(), (dim+1)%NB_DIM
                            )
                        } else if gr < ge {
                            Self::mut_rec_search_min_guide(
                                link.as_mut().unwrap().right_mut(), (dim+1)%NB_DIM
                            )
                        } else { (link, Some(dim)) }
                    }
                }
            }
        }
    }

    /// mutable recursive function to search a minimum node given a dimension
    fn mut_rec_search_minimum(link:&mut Link<T,Elt,NB_DIM>, dim:usize, target_dim:usize,)
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

    /// recursive search for a dominating node. Returns a node dominating the element if it exists
    fn rec_exists_dominating<'a>(link: &'a Link<T,Elt,NB_DIM>, elt:&Elt) -> Option<&'a Elt> {
        match link {
            None => None,
            Some(node) => {
                // if the element has a coordinate lower than the bound, return None
                // TODO also check if the node element can dominate. If not, skip the dominance check
                for (i,d) in elt.coordinates().enumerate() {
                    if d < node.bounds()[i].0 { return None; }
                }
                if node.elt().dominates(elt) {
                    Some(node.elt())
                }
                else {
                    match Self::rec_exists_dominating(node.left(), elt) {
                        Some(e) => Some(e),
                        None => { Self::rec_exists_dominating(node.right(), elt) }
                    }
                }
            }
        }
    }

    /// recursive update of bounds.
    /// This function should eventually be removed and the update done only when needed
    fn rec_update_bounds(link:&mut Link<T,Elt,NB_DIM>) {
        if link.is_some() {
            Self::rec_update_bounds(link.as_mut().unwrap().left_mut());
            Self::rec_update_bounds(link.as_mut().unwrap().right_mut());
            if let Some(node) = link {
                node.update_bounds();
            }
        }
    }
}



#[cfg(test)]
pub mod test {
    use crate::priority_queue::util::CartesianParetoElement;

    use super::*;

    #[test]
    pub fn test_insert_then_pop_min() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert(CartesianParetoElement::new([0,10]));
        front.insert(CartesianParetoElement::new([10,5]));
        front.insert(CartesianParetoElement::new([20,0]));
        // front.pretty_print();
        assert_eq!(front.peek_minimum().unwrap(), &CartesianParetoElement::new([0,10]));
        assert_eq!(front.peek_minimum_guide().unwrap(), 10);
    }

    #[test]
    pub fn test_remove_empty() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.pop_minimum().is_none());
    }

    #[test]
    pub fn test_remove_1() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert(CartesianParetoElement::new([10,10]));
        assert!(front.pop_minimum().is_some());
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_remove_2() { // delete center, right, left
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert(CartesianParetoElement::new([10,10]));
        front.insert(CartesianParetoElement::new([5,5]));
        front.insert(CartesianParetoElement::new([20,20]));
        assert_eq!(front.pop_minimum(), Some(CartesianParetoElement::new([5,5])));
    }

    #[test]
    pub fn test_insert_no_domination() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        front.insert(CartesianParetoElement::new([5,10]));
        front.insert(CartesianParetoElement::new([10,6]));
        assert_eq!(front.pop_minimum().unwrap(), CartesianParetoElement::new([5,10]));
        assert_eq!(front.pop_minimum().unwrap(), CartesianParetoElement::new([10,6]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_existing() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert!(!front.insert(CartesianParetoElement::new([10,10])));
        assert_eq!(front.pop_minimum().unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

    #[test]
    pub fn test_insert_domination_by_inserted() {
        let mut front:KDTreeFront<u32, CartesianParetoElement<2>, 2> = KDTreeFront::default();
        assert!(front.insert(CartesianParetoElement::new([10,10])));
        assert!(front.insert(CartesianParetoElement::new([5,5])));
        assert_eq!(front.pop_minimum().unwrap(), CartesianParetoElement::new([5,5]));
        assert!(front.is_empty());
    }

}