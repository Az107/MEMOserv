// Written by Alberto Ruiz 2024-03-08
// [Unfinished] module to index and create search structures
//
// The Binary tree will be used to index the documents
// and provide a fast search

use std::collections::btree_map::Values;



struct BinaryNode {
    value: u64,
    l: Option<Box<BinaryNode>>, // value > l.value
    r: Option<Box<BinaryNode>> // value < r.value
}

impl BinaryNode {
    fn new(value: u64) -> Self {
        BinaryNode {
            value,
            l: None,
            r: None
        }
    }

    fn insert(&mut self, new_value: u64) {
        if new_value < self.value {
            match self.l {
                Some(ref mut left) => left.insert(new_value),
                None => self.l = Some(Box::new(BinaryNode::new(new_value))),
            }
        } else {
            match self.r {
                Some(ref mut right) => right.insert(new_value),
                None => self.r = Some(Box::new(BinaryNode::new(new_value))),
            }
        }
    }
}

struct BinaryTree {
    trunc: Option<BinaryNode>
}


impl BinaryTree {
    fn new() -> BinaryTree {
        BinaryTree { 
            trunc: None
         }
    }

    fn add(&mut self, v: u64) {
        match self.trunc {
            Some(ref mut node) => node.insert(v),
            None => self.trunc = Some(BinaryNode::new(v))
        }
    }
}



#[cfg(test)]
mod test {
    use super::BinaryTree;


    #[test]
    fn test_tree() {
        let mut tree = BinaryTree::new();
        tree.add(5);
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.unwrap().value == 5);
    }

    #[test]
    fn test_tree_add() {
        let mut tree = BinaryTree::new();
        tree.add(5);
        tree.add(10);
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.as_ref().unwrap().value == 5);
        assert!(tree.trunc.as_ref().unwrap().r.is_some());
        assert!(tree.trunc.as_ref().unwrap().r.as_ref().unwrap().value == 10);

    }
}
