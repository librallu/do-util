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
        let mut index:usize = 0;
        let mut current_node = &mut self.root;
        while index < s.len() {
            let e = s[index];
            if current_node.is_none() { // populate the node if needed
                *current_node = Some(Box::new(Node {
                    contains_set:false,
                    children:vec![],
                    nb_children:0,
                }));
            }
            // extend the children until e is found
            let n = current_node.as_mut().unwrap();
            while n.children.len() <= e.into() {
                n.children.push(None);
            }
            // update current node and index
            if n.children[e.into()].is_none() {
                n.nb_children += 1;
            }
            current_node = &mut n.children[e.into()];
            index += 1;
        }
        if current_node.is_none() { // populate the node if needed
            *current_node = Some(Box::new(Node {
                contains_set:false,
                children:vec![],
                nb_children:0
            }));
        }
        if current_node.as_ref().unwrap().contains_set { return false; }
        current_node.as_mut().unwrap().contains_set = true;
        true
    }

    fn remove(&mut self, s:&[T]) -> bool {
        Self::remove_rec(&mut self.root, s, 0)
    }

    fn find_subsets(&self, s:&[T]) -> Self::SubsetIterator {
        Self::rec_find_subsets(&self.root, s, 0, Vec::new()).into_iter()
    }

    fn find_supersets(&self, s:&[T]) -> Self::SupersetIterator {
        Self::rec_find_supersets(&self.root, s, 0, Vec::new()).into_iter()
    }

    fn contains(&self, s:&[T]) -> bool {
        let mut current_node = &self.root;
        for e in s {
            match current_node {
                None => { return false; }
                Some(n) => {
                    if n.children.len() < (*e).into() { return false; }
                    current_node = &n.children[(*e).into()]
                }
            }
        }
        match current_node {
            None => { false }
            Some(n) => { n.contains_set }
        }
    }
}

impl Default for TrieSetStore {
    fn default() -> Self {
        Self { root: Some(Box::new(Node {
            contains_set: false,
            children: Vec::new(),
            nb_children: 0
        })) }
    }
}


impl TrieSetStore {

    fn rec_find_subsets<T:Copy+Eq+Into<usize>>(node:&Option<Box<Node>>, e:&[T], index:usize, selected:Vec<T>)
    -> Vec<Vec<T>> {
        let mut res = Vec::new();
        match node {
            None => { return res; }
            Some(n) => {
                if n.contains_set { res.push(selected.clone()); }
                for i in index..e.len() {
                    let v = e[i];
                    if v.into() < n.children.len() && n.children[v.into()].is_some() {
                        let mut new_selected = selected.clone();
                        new_selected.push(v);
                        let mut rec_res = Self::rec_find_subsets(
                            &n.children[v.into()], e, i, new_selected.clone()
                        );
                        res.append(&mut rec_res);
                    }
                }
            }
        }
        res
    }

    fn rec_find_supersets<T:Copy+Eq+Into<usize>+From<usize>>(node:&Option<Box<Node>>, e:&[T], index:usize, selected:Vec<T>)
    -> Vec<Vec<T>> {
        let mut res = Vec::new();
        match node {
            None => { return res; },
            Some(n) => {
                if index >= e.len() { // find all sets below
                    if n.contains_set { res.push(selected.clone()); }
                    for (i,c) in n.children.iter().enumerate() {
                        let mut new_selected = selected.clone();
                        new_selected.push(i.into());
                        let mut rec_res = Self::rec_find_supersets(c,e,index,new_selected);
                        res.append(&mut rec_res);
                    }
                } else { // iterate over all values less or equal than e[index]
                    let v = e[index];
                    for i in 0..v.into()+1 {
                        if i < n.children.len() && n.children[i].is_some() {
                            let mut new_selected = selected.clone();
                            new_selected.push(i.into());
                            let mut rec_res = if i == v.into() {
                                Self::rec_find_supersets(
                                    &n.children[i], e, index+1, new_selected.clone()
                                )
                            } else {
                                Self::rec_find_supersets(
                                    &n.children[i], e, index, new_selected.clone()
                                )
                            };
                            res.append(&mut rec_res);
                        }
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
                res += format!("\t{} -> {} [label=\"{}\"];\n", id, current_id, v).as_str();
                let (tmp_str, next_id) = Self::node_to_graphviz(c, current_id);
                res += tmp_str.as_str();
                current_id = next_id;
            }
        }
        (res, current_id)
    }

    /// removes e[index:] from node
    fn remove_rec<T:Copy+Eq+Into<usize>>(node:&mut Option<Box<Node>>, e:&[T], index:usize) -> bool {
        if index == e.len() { // right spot is found
            match node {
                None => { return false; }
                Some(n) => {
                    if n.contains_set {
                        if n.nb_children == 0 { // if no children, remove the node,
                            *node = None;
                        } else {// otherwise, just remove the set of the node
                            n.contains_set = false;
                        }
                        return true;
                    } else { return false; } // the right spot does not contain a set
                }
            }
        } else if index >= e.len() { return false; } // no element found
        match node {
            None => { false }
            Some(n) => {
                let v:usize = e[index].into();
                if n.children.len() <= v { false }
                else { // call recursive remove
                    let child_is_some = n.children[v].is_some();
                    let res = Self::remove_rec(&mut n.children[v], e, index+1);
                    if child_is_some && n.children[v].is_none() { // child removed
                        n.nb_children -= 1;
                        if n.nb_children == 0 {
                            *node = None;
                        }
                    }
                    res
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