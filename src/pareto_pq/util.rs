use crate::pareto_pq::{ParetoElement};


/// Simple 2D point for the pareto front
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct CartesianParetoElement<const NB_DIM:usize> {
    /// coordinates
    coords:[u16;NB_DIM]
}

impl<const NB_DIM:usize> ParetoElement<u16> for CartesianParetoElement<NB_DIM> {
    type CoordIterator = std::array::IntoIter<u16,NB_DIM>;

    fn coordinates(&self) -> Self::CoordIterator { self.coords.into_iter() }

    fn dominates(&self, other:&Self) -> bool {
        for i in 0..NB_DIM {
            if self.coords[i] > other.coords[i] { return false; }
        }
        true
    }

    fn nb_dimensions(&self) -> usize { NB_DIM }

    fn kth(&self, k:usize) -> u16 { self.coords[k] }
}

impl<const NB_DIM:usize> CartesianParetoElement<NB_DIM> {
    /// constructor taking two coordinates
    pub fn new(coords:[u16;NB_DIM]) -> Self {
        Self { coords }
    }
}



#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_strict_dominance() {
        let e1 = CartesianParetoElement::new([0]);
        let e2 = CartesianParetoElement::new([1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_nonstrict_dominance() {
        let e1 = CartesianParetoElement::new([0]);
        let e2 = CartesianParetoElement::new([0]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_non_dominance() {
        let e1 = CartesianParetoElement::new([1]);
        let e2 = CartesianParetoElement::new([0]);
        assert!(!e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_dominance() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([2,1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_dominance2() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([1,1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_non_dominance() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([0,1]);
        assert!(!e1.dominates(&e2));
    }
}