use super::SetStore;


/// ListSetStore
/// 
/// maintains the list of sets using a vector.
/// - insertion: O(1)
/// - deletion: O(n)
/// - find_subsets: O(n)
/// - find_supersets: O(n)
#[derive(Debug)]
pub struct ListSetStore<T> {
    list:Vec<Vec<T>>
}

impl<T:Copy+Eq+Ord> SetStore<T> for ListSetStore<T> {
    type SubsetIterator = std::vec::IntoIter<Vec<T>>;
    type SupersetIterator = std::vec::IntoIter<Vec<T>>;

    /// In this implementation, we do not check if the element was already existing in the set.
    /// Another implementation would use a HashMap to store which elements were inserted or not.
    fn insert(&mut self, s:&[T]) -> bool {
        self.list.push(s.iter().copied().collect());
        true
    }

    fn remove(&mut self, s:&[T]) -> bool {
        let previous_size = self.list.len();
        self.list = self.list.iter().filter(|e| *e != s).cloned().collect();
        let next_size = self.list.len();
        next_size < previous_size
    }

    fn find_subsets(&self, s:&[T]) -> Self::SubsetIterator {
        self.list.iter().filter(|e| Self::is_subset(e, s)).cloned()
            .collect::<Vec<Vec<T>>>().into_iter()
    }

    fn find_supersets(&self, s:&[T]) -> Self::SupersetIterator {
        self.list.iter().filter(|e| Self::is_subset(s, e)).cloned()
            .collect::<Vec<Vec<T>>>().into_iter()
    }

    fn contains(&self, s:&[T]) -> bool { self.list.iter().any(|e| e==s) }
}


impl<T> ListSetStore<T> where T:Ord {

    /// returns true if a is a subset of b
    /// 
    /// It assumes the sequences to be sorted
    pub fn is_subset(a:&[T], b:&[T]) -> bool {
        if a.len() > b.len() { return false; }
        let mut index_b:usize = 0;
        for e in a {
            while index_b < b.len() && &b[index_b] < e { index_b += 1 }
            if index_b == b.len() || &b[index_b] > e { return false; }
        }
        true
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_subset() {
        assert!(ListSetStore::<usize>::is_subset(&[], &[]));
        assert!(ListSetStore::<usize>::is_subset(&[], &[1]));
        assert!(ListSetStore::<usize>::is_subset(&[], &[1,2]));
        assert!(ListSetStore::<usize>::is_subset(&[1], &[1,2]));
        assert!(ListSetStore::<usize>::is_subset(&[1,2], &[1,2]));
        assert!(ListSetStore::<usize>::is_subset(&[1,4], &[1,2,3,4]));
        assert!(!ListSetStore::<usize>::is_subset(&[1,5], &[1,2,3,4]));
        assert!(!ListSetStore::<usize>::is_subset(&[1,3], &[1,2,4]));
        assert!(!ListSetStore::<usize>::is_subset(&[1,3], &[3,4,5]));
    }
}