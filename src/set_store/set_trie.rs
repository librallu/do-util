use crate::set_store::SetStore;

/// Trie node
#[derive(Debug)]
struct Node {
    /// true iff the node contains a set
    contains_set:bool,
    /// list of children in the node
    children:Vec<Option<Box<Node>>>,
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

impl<T:Copy+Eq+Into<usize>> SetStore<T> for TrieSetStore {
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
                    children:vec![]
                }));
            }
            // extend the children until e is found
            let n = current_node.as_mut().unwrap();
            while n.children.len() <= e.into() {
                n.children.push(None);
            }
            // update current node and index
            current_node = &mut n.children[e.into()];
            index += 1;
        }
        if current_node.is_none() { // populate the node if needed
            *current_node = Some(Box::new(Node {
                contains_set:false,
                children:vec![]
            }));
        }
        if current_node.as_ref().unwrap().contains_set { return false; }
        current_node.as_mut().unwrap().contains_set = true;
        true
    }

    fn remove(&mut self, s:&[T]) -> bool {
        todo!()
    }

    fn find_subsets(&self, s:&[T]) -> Self::SubsetIterator {
        todo!()
    }

    fn find_supersets(&self, s:&[T]) -> Self::SupersetIterator {
        todo!()
    }

    fn contains(&self, s:&[T]) -> bool {
        todo!()
    }
}

impl Default for TrieSetStore {
    fn default() -> Self {
        Self { root: Some(Box::new(Node { contains_set: false, children: Vec::new() })) }
    }
}


impl TrieSetStore {
    pub fn to_graphviz(&self) -> String {
        let mut res = "digraph {\n".to_string();
        if let Some(n) = &self.root {
            res += Self::node_to_graphviz(&n, 0).0.as_str();
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
                res += format!("\t{} -> {} [label=\"{}\"]", id, current_id, v).as_str();
                let (tmp_str, next_id) = Self::node_to_graphviz(c, current_id);
                res += tmp_str.as_str();
                current_id = next_id;
            }
        }
        (res, current_id)
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
        println!("{}", trie.to_graphviz());
    }
}