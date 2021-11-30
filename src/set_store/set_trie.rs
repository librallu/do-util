use crate::set_store::SetStore;

/// Trie node
#[derive(Debug)]
struct Node {
    /// true iff the node contains a set
    contains_set:bool,
    /// list of children in the node
    children:Vec<Option<Box<Node>>>,
    /// number of "active" children
    nb_children:usize,
    /// offset of the children
    offset:usize,
}

impl Node {
    /// creates a new empty node
    pub fn new() -> Self {
        Node {
            contains_set:false,
            children:vec![],
            nb_children:0,
            offset: 0
        }
    }

    /// returns true iff the given value is out of bounds
    pub fn is_out_of_bounds(&self, v:usize) -> bool {
        v < self.offset || v >= self.offset+self.children.len()
    }

    /// returns the child of the node using 
    pub fn child(&self, v:usize) -> &Option<Box<Node>> {
        &self.children[v-self.offset]
    }

    /// returns the (mutable) child of the node using 
    pub fn child_mut(&mut self, v:usize) -> &mut Option<Box<Node>> {
        &mut self.children[v-self.offset]
    }

    /// adds a child to the node if it does not exist.
    /// returns it
    pub fn add_child(&mut self, v:usize) -> &mut Node{
        assert!(v >= self.offset);
        // add new children
        while self.offset+self.children.len() <= v {
            self.children.push(None);
        }
        // update current node and index
        let children_index = v-self.offset;
        if self.children[children_index].is_none() {
            self.nb_children += 1;
            self.children[children_index] = Some(Box::new(Node {
                contains_set:false,
                children:vec![],
                nb_children:0,
                offset: v+1
            }));
        }
        &mut (*self.children[children_index].as_mut().unwrap())
    }

    /// removes a child indexed by v
    fn remove_child(&mut self, v:usize) -> bool {
        if self.is_out_of_bounds(v) { return false; }
        if v-self.offset >= self.children.len() { return false; }
        self.children[v-self.offset] = None;
        true
    }

    /// adds a set to the node. Returns true if the set was successfully added
    fn add_set(&mut self) -> bool {
        if self.contains_set {
            false
        } else {
            self.contains_set = true;
            true
        }
    }

    /// returns true iff the node contains a set
    fn has_set(&self) -> bool { self.contains_set }

    /// removes the set if it exists
    fn remove_set(&mut self) {
        self.contains_set = false;
    }

    /// returns the number of children
    fn nb_children(&self) -> usize { self.children.len() }
}

/// Set Trie.
/// 
/// Implements a trie data-structure to maintain sets, and perform efficient sub-set/super-set
/// queries.
#[derive(Debug)]
pub struct TrieSetStore {
    /// root node of the tree
    root: Option<Box<Node>>
}

impl<T:Copy+Eq+Into<usize>+From<usize>> SetStore<T> for TrieSetStore {
    type SubsetIterator = std::vec::IntoIter<Vec<T>>;
    type SupersetIterator = std::vec::IntoIter<Vec<T>>;

    fn insert(&mut self, s:&[T]) -> bool {
        let mut current_node = &mut **self.root.as_mut().unwrap();
        for v in s {
            current_node = current_node.add_child((*v).into());
        }
        current_node.add_set()
    }

    fn remove(&mut self, s:&[T]) -> bool {
        Self::remove_rec(self.root.as_mut().unwrap(), s, 0)
    }

    fn find_subsets(&self, s:&[T]) -> Self::SubsetIterator {
        Self::rec_find_subsets(
            self.root.as_ref().unwrap(),
            s,
            0,
            Vec::new()
        ).into_iter()
    }

    fn find_supersets(&self, s:&[T]) -> Self::SupersetIterator {
        Self::rec_find_supersets(
            self.root.as_ref().unwrap(),
            s,
            0,
            Vec::new()
        ).into_iter()
    }

    fn contains(&self, s:&[T]) -> bool {
        let mut current_node = self.root.as_ref().unwrap();
        for v in s {
            if current_node.is_out_of_bounds((*v).into()) { return false; }
            match current_node.child((*v).into()) {
                None => { return false; },
                Some(n) => { current_node = n; },
            }
        }
        current_node.has_set()
    }
}

impl Default for TrieSetStore {
    fn default() -> Self {
        Self { root: Some(Box::new(Node::new())) }
    }
}


impl TrieSetStore {

    fn rec_find_subsets<T:Copy+Eq+Into<usize>+From<usize>>(node:&Node, e:&[T], index:usize, selected:Vec<T>)
    -> Vec<Vec<T>> {
        let mut res = Vec::new();
        if node.contains_set { res.push(selected.clone()); }
        for i in index..e.len() {
            let v = e[i];
            if node.is_out_of_bounds(v.into()) { continue; }
            match node.child(v.into()) {
                None => { continue; }
                Some(child) => {
                    let mut new_selected = selected.clone();
                    new_selected.push(i.into());
                    let mut rec_res = Self::rec_find_subsets(
                        child, e, i, new_selected.clone()
                    );
                    res.append(&mut rec_res);
                }
            }
        }
        res
    }

    fn rec_find_supersets<T:Copy+Eq+Into<usize>+From<usize>>(node:&Node, e:&[T], index:usize, selected:Vec<T>)
    -> Vec<Vec<T>> {
        let mut res = Vec::new();
        if index >= e.len() { // find all sets below
            if node.contains_set { res.push(selected.clone()); }
            for (i,c) in node.children.iter().enumerate()
            .filter(|(_,c)| c.is_some()) {
                let mut new_selected = selected.clone();
                new_selected.push(T::from(i+node.offset));
                let mut rec_res = Self::rec_find_supersets(c.as_ref().unwrap(),e,index,new_selected);
                res.append(&mut rec_res);
            }
        } else { // iterate over all values less or equal than e[index]
            let v = e[index];
            for i in 0..v.into()+1 { // search for all elements below the value v (included)
                if node.is_out_of_bounds(i) { continue; }
                match node.child(i) {
                    None => { continue; },
                    Some(child) => {
                        let mut new_selected = selected.clone();
                        new_selected.push(i.into());
                        let mut rec_res = if i == v.into() {
                            Self::rec_find_supersets(
                                child, e, index+1, new_selected.clone()
                            )
                        } else {
                            Self::rec_find_supersets(
                                child, e, index, new_selected.clone()
                            )
                        };
                        res.append(&mut rec_res);
                    }
                }
            }
        }
        res
    }

    /// generates the graphviz representation of the trie.
    pub fn to_graphviz(&self) -> String {
        let mut res = "digraph {\n".to_string();
        if let Some(n) = &self.root {
            res += Self::node_to_graphviz(n, 0).0.as_str();
        }
        res += "}";
        res
    }

    fn node_to_graphviz(node:&Node, id:usize) -> (String,usize) {
        let shape = if node.contains_set { "doublecircle" } else { "circle" };
        let mut res = format!("\t{} [label=\"\",shape=\"{}\"];\n", id, shape);
        let mut current_id = id+1;
        for (v,child) in node.children.iter().enumerate() {
            if let Some(c) = child {
                res += format!("\t{} -> {} [label=\"{}\"];\n",
                    id, current_id, v+child.as_ref().unwrap().offset
                ).as_str();
                let (tmp_str, next_id) = Self::node_to_graphviz(c, current_id);
                res += tmp_str.as_str();
                current_id = next_id;
            }
        }
        (res, current_id)
    }

    /// removes e[index:] from node
    fn remove_rec<T:Copy+Eq+Into<usize>>(node:&mut Node, e:&[T], index:usize) -> bool {
        if index == e.len() { // right spot is found
            if node.has_set() { node.remove_set(); true }
            else { false }
        } else {
            // check if the next element exists, and perform a recursive remove
            if node.is_out_of_bounds(e[index].into()) { return false; }
            match node.child_mut(e[index].into()) {
                None => { false }
                Some(child) => { // if the child exists, recursive call
                    let res = Self::remove_rec(&mut **child, e, index+1);
                    if !res { return false; }
                    // if an element was removed, check its children, and possibly remove it
                    if child.nb_children() == 0 && !child.has_set() {
                        node.remove_child(e[index].into());
                    }
                    true
                }
            }
        }
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_insert_emptyset() {
        let mut trie = TrieSetStore::default();
        // println!("{}", trie.to_graphviz());
        let s:Vec<usize> = vec![];
        trie.insert(&s);
        // println!("{}", trie.to_graphviz());
    }

    #[test]
    fn test_insert() {
        let mut trie = TrieSetStore::default();
        let a:Vec<usize> = vec![1,2,3];
        let b:Vec<usize> = vec![1,3];
        trie.insert(&a);
        trie.insert(&b);
        // println!("{}", trie.to_graphviz());
    }

    #[test]
    fn test_contains() {
        let mut trie = TrieSetStore::default();
        let a:Vec<usize> = vec![1,2,3];
        let b:Vec<usize> = vec![1,3];
        trie.insert(&a);
        trie.insert(&b);
        let c:Vec<usize> = vec![1];
        let d:Vec<usize> = vec![3];
        assert!(!trie.contains(&c));
        assert!(!trie.contains(&d));
        assert!(trie.contains(&a));
        assert!(trie.contains(&b));
        // println!("{}", trie.to_graphviz());
    }

    #[test]
    fn test_remove() {
        let mut trie = TrieSetStore::default();
        let a:Vec<usize> = vec![1,2,3];
        let b:Vec<usize> = vec![1,3];
        trie.insert(&a);
        trie.insert(&b);
        assert!(trie.remove(&b));
        assert!(trie.remove(&a));
    }

    #[test]
    fn test_subset() {
        let mut trie = TrieSetStore::default();
        let a:Vec<usize> = vec![1,2,3];
        let b:Vec<usize> = vec![1,3];
        let c:Vec<usize> = vec![1];
        trie.insert(&a);
        trie.insert(&b);
        trie.insert(&c);
        // println!("{}", trie.to_graphviz());
        assert_eq!(trie.find_subsets(&b).count(), 2);
        // println!("{:?}", trie.find_subsets(&b));
    }

    #[test]
    fn test_superset() {
        let mut trie = TrieSetStore::default();
        let a:Vec<usize> = vec![1,2,3];
        let b:Vec<usize> = vec![1,3];
        let c:Vec<usize> = vec![1];
        trie.insert(&a);
        trie.insert(&b);
        trie.insert(&c);
        assert_eq!(trie.find_supersets(&b).count(), 2);
        // println!("{}", trie.to_graphviz());
        // println!("{:?}", trie.find_supersets(&b));
    }

}