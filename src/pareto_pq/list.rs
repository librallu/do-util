use crate::pareto_pq::{ParetoElement, ParetoFront};

/// Simple pareto front that stores element in a list
#[derive(Debug)]
pub struct ListParetoFront<Elm> {
    elements:Vec<Elm>,
}

impl<T, Elt> ParetoFront<T,Elt>
for ListParetoFront<Elt> where T:Ord, Elt:ParetoElement<T>+Eq {
    fn query(&self, min_bound:&[T], max_bound:&[T]) -> Vec<&Elt> {
        self.elements.iter().filter(|elt| {
            elt.coordinates().enumerate().all(|(i,v)| {
                min_bound[i] <= v && max_bound[i] >= v
            })
        }).collect()
    }

    fn pop_minimum_element(&mut self, dim:usize) -> Option<Elt> {
        if self.elements.is_empty() { return None; }
        let min_pos = self.elements.iter().enumerate()
            .min_by_key(|(_,elt)| elt.kth(dim))
            .map(|(pos,_)| pos).unwrap();
        Some(self.elements.swap_remove(min_pos))
    }

    fn peek_minimum_element(&mut self, dim:usize) -> Option<&Elt> {
        if self.elements.is_empty() { return None; }
        let min_pos = self.elements.iter().enumerate()
            .min_by_key(|(_,elt)| elt.kth(dim))
            .map(|(pos,_)| pos).unwrap();
        Some(&self.elements[min_pos])
    }

    fn find_dominating(&self, elt:&Elt) -> Option<&Elt> {
        self.elements.iter().find(|e| e.dominates(elt))
    }

    fn insert(&mut self, elt:Elt) -> bool {
        match self.find_dominating(&elt) {
            None => { // if the current element is not dominated, remove the ones dominated by it
                self.elements.retain(|e| !elt.dominates(e) );
                self.elements.push(elt);
                true
            },
            Some(_) => false
        }
        
    }

    fn remove(&mut self, elt:&Elt) -> bool {
        let mut index = None;
        for (i,e) in self.elements.iter().enumerate() {
            if elt == e { index = Some(i); break; }
        }
        match index {
            None => false,
            Some(i) => {
                self.elements.swap_remove(i);
                true
            }
        }
    }

    fn create_empty() -> Self {
        Self::default()
    }
}

impl<Elt> Default for ListParetoFront<Elt> {
    fn default() -> Self {
        Self { elements: Default::default() }
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    use crate::pareto_pq::util::CartesianParetoElement;

    #[test]
    pub fn test_some_inserts() {
        let mut front = ListParetoFront::<CartesianParetoElement<2>>::default();
        assert!(front.insert(CartesianParetoElement::<2>::new([1,0])));
        assert!(front.insert(CartesianParetoElement::<2>::new([0,1])));
        assert!(!front.insert(CartesianParetoElement::<2>::new([1,1])));
    }
}