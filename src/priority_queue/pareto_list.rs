use std::marker::PhantomData;

use crate::priority_queue::{GuidedElement, ParetoElement, PriorityQueue};

use super::ParetoFront;

/// Simple pareto front that stores element in a list
#[derive(Debug)]
pub struct ListParetoFront<T,Elt> {
    elements:Vec<Elt>,
    phantom_t:PhantomData<T>,
}

impl<T,Elt> PriorityQueue<T,Elt> for ListParetoFront<T,Elt>
where T:Ord, Elt:ParetoElement<T>+GuidedElement<T> {
    fn peek_min(&self) -> Option<&Elt> {
        self.elements.iter().min_by_key(|e| e.guide())
    }

    fn peek_max(&self) -> Option<&Elt> {
        self.elements.iter().max_by_key(|e| e.guide())
    }

    fn pop_min(&mut self) -> Option<Elt> {
        if self.elements.is_empty() { return None; }
        let min_pos = self.elements.iter().enumerate()
            .min_by_key(|(_,elt)| elt.guide())
            .map(|(pos,_)| pos).unwrap();
        Some(self.elements.swap_remove(min_pos))
    }

    fn pop_max(&mut self) -> Option<Elt> {
        if self.elements.is_empty() { return None; }
        let min_pos = self.elements.iter().enumerate()
            .max_by_key(|(_,elt)| elt.guide())
            .map(|(pos,_)| pos).unwrap();
        Some(self.elements.swap_remove(min_pos))
    }

    fn insert(&mut self, elt:Elt) -> bool {
        match self.find_dominating(&elt) {
            None => { // if the current element is not dominated, remove the ones dominated by it
                self.elements.retain(|e| !elt.dominates(e) );
                self.elements.push(elt);
                true
            },
            Some(_) => false // do not insert as we found some dominating element
        }
    }
}

impl<T,Elt> ParetoFront<T,Elt> for ListParetoFront<T,Elt>
where T:Ord, Elt:ParetoElement<T> {
    fn find_dominating(&self, elt:&Elt) -> Option<&Elt> {
        self.elements.iter().find(|e| e.dominates(elt))
    }
}


impl<Elt,T> Default for ListParetoFront<T,Elt> {
    fn default() -> Self {
        Self { elements: Default::default(), phantom_t:PhantomData::default() }
    }
}



#[cfg(test)]
pub mod test {
    use super::*;

    use crate::priority_queue::util::CartesianParetoElement;

    #[test]
    pub fn test_some_inserts() {
        let mut front = ListParetoFront::<u32, CartesianParetoElement<2>>::default();
        assert!(front.insert(CartesianParetoElement::<2>::new([1,0])));
        assert!(front.insert(CartesianParetoElement::<2>::new([0,1])));
        assert!(!front.insert(CartesianParetoElement::<2>::new([1,1])));
    }
}