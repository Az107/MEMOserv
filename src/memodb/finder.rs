// Written by Alberto Ruiz 2024-03-08
// [Unfinished] module to index and create search structures
//
// The Binary tree will be used to index the documents
// and provide a fast search

use std::collections::HashMap;
use uuid::Uuid;
use super::data_type::DataType;


struct BinaryNode {
    value: u64,
    pointers: Vec<Uuid>,
    l: Option<Box<BinaryNode>>, // value > l.value
    r: Option<Box<BinaryNode>> // value < r.value
}

impl BinaryNode {
    fn new(value: u64, id: Uuid) -> Self {
        let mut pointers = Vec::new();
        pointers.push(id);
        BinaryNode {
            value,
            pointers: pointers,
            l: None,
            r: None
        }
    }

    fn find(&self, value: u64) -> Option<Vec<Uuid>> {
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

    fn insert(&mut self, new_value: u64, index: Uuid) {
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
            if !self.pointers.contains(&index) {
                self.pointers.push(index);
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

    fn add(&mut self, v: u64, index: Uuid) {
        match self.trunc {
            Some(ref mut node) => node.insert(v, index),
            None => self.trunc = Some(BinaryNode::new(v, index))
        }
    }

    fn find(&self, value : u64) -> Option<Vec<Uuid>> {
        match &self.trunc {
            Some(node) => node.find(value),
            None => None
        }
    }
}

fn fnv1(s: &str) -> u64 {

    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash: u64 = FNV_OFFSET_BASIS;

    for byte in s.as_bytes() {
        hash = hash.wrapping_mul(FNV_PRIME); 
        hash ^= *byte as u64;
    }

    hash


}

pub struct DataFinder {
    forest: HashMap<String,BinaryTree>,
}

impl DataFinder {
    pub fn new() -> Self {
        DataFinder {
            forest: HashMap::new()
        }
    }

    pub fn add(&mut self, key: &str, value: DataType, index: Uuid) {
        let value = value.to_string();
        let value = fnv1(value.as_str());
        let tree = self.forest.get_mut(key);
        match tree {
            Some(tree) => { tree.add(value, index); },
            None => {
                let mut tree = BinaryTree::new();
                tree.add(value, index);
                self.forest.insert(key.to_string(), tree);

            }
        }
    }

    pub fn find(&mut self, key: &str, value: &str) -> Option<Vec<Uuid>> {
        let tree = self.forest.get(key);
        if tree.is_none() {
            return None
        }
        let tree = tree.unwrap();
        let value = fnv1(value);

        tree.find(value)
    }


}


struct TextProcessor{
    wordCound: HashMap<String,u64>
}

impl TextProcessor {
    pub fn new() -> Self {
        TextProcessor {
            wordCound: HashMap::new()
        }
    }

    pub fn addText(&mut self, text: String) {
        let v: Vec<&str> = text.split(' ').collect();
    }
}



#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::memodb::finder::fnv1;

    use super::BinaryTree;


    #[test]
    fn test_tree() {
        let mut tree = BinaryTree::new();
        tree.add(5,Uuid::new_v4());
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.unwrap().value == 5);
    }

    #[test]
    fn test_tree_add() {
        let mut tree = BinaryTree::new();
        tree.add(5,Uuid::new_v4());
        tree.add(10,Uuid::new_v4());
        assert!(tree.trunc.is_some());
        assert!(tree.trunc.as_ref().unwrap().value == 5);
        assert!(tree.trunc.as_ref().unwrap().r.is_some());
        assert!(tree.trunc.as_ref().unwrap().r.as_ref().unwrap().value == 10);

    }


    #[test]
    fn test_tree_find() {
        let mut tree = BinaryTree::new();
        tree.add(5,Uuid::new_v4());
        let uno = Uuid::new_v4();
        tree.add(5,uno);
        tree.add(5,uno);
        tree.add(10,Uuid::new_v4());
        tree.add(12,Uuid::new_v4());
        assert!(tree.find(5).is_some());
        assert!(!tree.find(5).unwrap().is_empty());
        assert!(tree.find(5).unwrap().len() == 2);
        assert!(tree.find(10).is_some());
        assert!(!tree.find(10).unwrap().is_empty());
        assert!(tree.find(10).unwrap().len() == 1);
        assert!(tree.find(15).is_none());
    }


    #[test]
    fn test_fnv1() {
        assert!(fnv1("a") == 12638153115695167422);
        assert!(fnv1("thisIsaLongStringToTestTheHashFunction") == 4027512090620836661)
    }
}
