// Writen by Alberto Ruiz 2024-03-08
// The collection module will provide the collection of documents for the MEMOdb
// The collection will store the documents in memory and provide a simple API to interact with them
// The Document will be a HashMap<String, DataType> 

use uuid::Uuid;
use std::collections::HashMap;
use super::data_type::DataType;

const ID: &str = "ID";

//create a trait based on HashMap<String,DataType>
// and impl especial methods for it
pub type Document = HashMap<String, DataType>;


pub trait DocumentStruct {
  fn to_document(&self) -> Document;
  fn from_document(document: &Document) -> Self;
}

pub trait DocumentJson {
  fn to_json(&self) -> String;
  fn from_json(json: &str) -> Self;
}

impl DocumentJson for Document {
  fn to_json(&self) -> String {
    let mut json = String::from("{");
    for (key, value) in self.iter() {
      json.push_str(&format!("\"{}\":{},", key, value.to_json()));
    }
    json.pop();
    json.push('}');
    json
  }
  fn from_json(json: &str) -> Self {
    let json = json.trim().trim_end();
    //remove all traling zero bytes
    let json = json.trim_matches(char::from(0));
    let json = &json[1..json.len()-1];

    //remove last bracket
    let mut document = Document::new();
    let json = json.split(',');
    for kv in json {
      println!("value: {}", kv);
      let mut kv = kv.split(':');
      let key = kv.next().unwrap().trim().replace("\"", "");
      let value = kv.next().unwrap().trim();
      if key == ID {
        //TODO check if value is a number otherwise return error ? or parse not as ID
        let value = value.parse::<Uuid>().unwrap();
        document.insert(key.to_string(), DataType::Id(value));
      } else {
        document.insert(key.to_string(), DataType::from_json(value));
      }
    }
    document
  
  }
}


//create a macro to create a document
#[macro_export]
macro_rules! doc {
  ( $( $key: expr => $value: expr ),* ) => {
    {
      use crate::memodb::data_type::DataType; // Add this line
      let mut map = crate::memodb::collection::Document::new();
      $(
        map.insert($key.to_string(), DataType::from($value)); // Update this line
      )*
      map
    }
  };
}





pub struct Collection {
  pub name: String,
  last_id: u32,
  pub(crate) data: Vec<Document>,
  id_table: HashMap<Uuid, usize>,
  //b_tree: BNode
}



impl Collection {
  pub fn new(name: String) -> Self {
    Collection {
      name: name,
      last_id: 0,
      data: Vec::new(),
      id_table: HashMap::new(),
      //b_tree: BNode::new(),
    }
  }

  fn update_index(&mut self) {
    self.id_table.clear();
    for (index, document) in self.data.iter().enumerate() {
      let id = document.get(ID).unwrap().to_id();
      self.id_table.insert(id, index);
    }
  }

  pub fn add(&mut self, document: Document) -> Uuid {
    let mut document = document;
    if !document.contains_key(ID) {
      let id = Uuid::new_v4();
      document.insert(ID.to_string(), DataType::Id(id));
    } else {
      let id = document.get(ID).unwrap().to_id();
      // if id exists replace id with new id
      if self.id_table.contains_key(&id) {
        document.remove(ID);
        let id = Uuid::new_v4();
        document.insert(ID.to_string(), DataType::Id(id));
      }
    }
    let id = document.get(ID).unwrap().to_id();
    self.data.push(document);
    self.id_table.insert(id, self.data.len() - 1);
    id
  }

  pub fn rm(&mut self, id: Uuid) {
    //self.data.remove(index);
    let index = self.get_index(id);
    self.data.swap_remove(index);
    self.update_index();
  }

  pub fn count(&self) -> usize {
    self.data.len()
  }

  fn _get(&self, index: usize) -> Option<&Document> {
    self.data.get(index)
  }

  fn get_index(&self, id: Uuid) -> usize {
    let id = DataType::Id(id);
    self.data.iter().position(|x| x.get(ID).unwrap() == &id).unwrap()
  }

  pub fn get_all(&self) -> &Vec<Document> {
    &self.data
    
   }

  fn _find_by_key(&self, key: &str) -> Vec<&Document> {
    self.data.iter().filter(|&x| x.contains_key(key)).collect()
  }

  fn _find_by_value(&self, key: &str, value: &DataType) -> Vec<&Document> {
    self.data.iter().filter(|&x| x.contains_key(key) && x.get(key).unwrap() == value).collect()
  }

  pub fn find(&self, args: HashMap<String, DataType>) -> Vec<&Document> {
    let mut result = Vec::new();
    for (key, value) in args.iter() {
      if key == ID {
        let id = value.to_id();
        let index = self.id_table.get(&id);
        match index {
          Some(index) => result.push(self._get(*index).unwrap()),
          None => continue
        }
      } else {
        result.append(&mut self._find_by_value(key, value));
      }
    }
    result
  }

  fn slow_get(&self, id: Uuid) -> Option<&Document> {
    let id = DataType::Id(id);
    self.data.iter().find(|&x| x.get(ID).unwrap() == &id)
  }

  pub fn get(&self, id: Uuid) -> Option<&Document> {
    let index = self.id_table.get(&id);
    match index {
      Some(index) => self.data.get(*index),
      None => self.slow_get(id)
    }
  }

}


//TEST
#[cfg(test)]
mod tests {
  use crate::memodb::collection::Collection;
  use crate::doc;

  #[test]
  fn test_collection() {
    let mut collection = Collection::new("users".to_string());
    collection.add(doc!(
      "name" => "John", 
      "age" => 25, 
      "isMarried" => false, 
      "birthDate" => "1995-01-01"
    ));
    assert!(collection._get(0).is_some());
  }
}
