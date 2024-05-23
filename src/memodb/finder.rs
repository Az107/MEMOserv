// Written by Alberto Ruiz 2024-03-08
// [Unfinished] module to index and create search structures
//
// The Binary tree will be used to index the documents
// and provide a fast search


struct BinaryNode {
    value: u64,
    pointers: Vec<u64>,
    l: Option<Box<BinaryNode>>, // value > l.value
    r: Option<Box<BinaryNode>> // value < r.value
}

impl BinaryNode {
    fn new(value: u64, pointer: u64) -> Self {
        let mut pointers = Vec::new();
        pointers.push(pointer);
        BinaryNode {
            value,
            pointers: pointers,
            l: None,
            r: None
        }
    }

    fn find(&self, value: u64) -> Option<Vec<u64>> {
        if value < self.value {
            return match &self.l {
                Some(left) => left.find(value),
                None => None,
            }
        } else if value > self.value {
            return match &self.r {
                Some(right) => right.find(value),
                None => None,
            }
        } else {
            Some(self.pointers.clone())
        }
    }

    fn insert(&mut self, new_value: u64, index: u64) {
        if new_value < self.value {
            match self.l {
                Some(ref mut left) => left.insert(new_value, index),
                None => self.l = Some(Box::new(BinaryNode::new(new_value,index))),
            }
        } else if new_value > self.value {
            match self.r {
                Some(ref mut right) => right.insert(new_value, index),
                None => self.r = Some(Box::new(BinaryNode::new(new_value, index))),
            }
        } else {
            self.pointers.push(index);
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

    fn add(&mut self, v: u64, index: u64) {
        match self.trunc {
            Some(ref mut node) => node.insert(v, index),
            None => self.trunc = Some(BinaryNode::new(v, index))
        }
    }

    fn find(&self, value : u64) -> Option<Vec<u64>> {
        match &self.trunc {
            Some(node) => node.find(value),
            None => None
        }
    }
}



#[cfg(test)]
mod test {
    use super::BinaryTree;


    #[test]
    fn test_tree() {
        let mut tree = BinaryTree::new();
        tree.add(5,0);
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.unwrap().value == 5);
    }

    #[test]
    fn test_tree_add() {
        let mut tree = BinaryTree::new();
        tree.add(5,0);
        tree.add(10,0);
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.as_ref().unwrap().value == 5);
        assert!(tree.trunc.as_ref().unwrap().r.is_some());
        assert!(tree.trunc.as_ref().unwrap().r.as_ref().unwrap().value == 10);

    }


    #[test]
    fn test_tree_find() {
        let mut tree = BinaryTree::new();
        tree.add(5,0);
        tree.add(5,1);
        tree.add(10,0);
        tree.add(12,0);
        assert!(tree.find(5).is_some());
        assert!(!tree.find(5).unwrap().is_empty());
        assert!(tree.find(5).unwrap().len() == 2);
        assert!(tree.find(10).is_some());
        assert!(!tree.find(10).unwrap().is_empty());
        assert!(tree.find(10).unwrap().len() == 1);
        assert!(tree.find(15).is_none());
    }
}
