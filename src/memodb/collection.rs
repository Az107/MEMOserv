use std::collections::HashMap;
use super::dataType::DataType;

const ID: &str = "ID";

//create a trait based on HashMap<String,DataType>
// and impl especial methods for it
pub type Document = HashMap<String, DataType>;


pub trait Document_struct {
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
    let mut json = json.split(',');
    for kv in json {
      println!("value: {}", kv);
      let mut kv = kv.split(':');
      let key = kv.next().unwrap().trim().replace("\"", "");
      let value = kv.next().unwrap().trim();
      document.insert(key.to_string(), DataType::from_json(value));
    }
    document
  
  }
}


//create a macro to create a document
#[macro_export]
macro_rules! doc {
  ( $( $key: expr => $value: expr ),* ) => {
    {
      use crate::memodb::dataType::DataType; // Add this line
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
  //idTable: HashMap<u32, usize>
}



impl Collection {
  pub fn new(name: String) -> Self {
    Collection {
      name: name,
      last_id: 0,
      data: Vec::new(),
      //idTable: HashMap::new()
    }
  }

  pub fn add(&mut self, document: Document) -> u32 {
    let mut document = document;
    if !document.contains_key(ID) {
      self.last_id += 1;
      document.insert(ID.to_string(), DataType::Id(self.last_id));
    }
    self.data.push(document);
    self.last_id
  }

  pub fn rm(&mut self, id: u32) {
    //self.data.remove(index);
    let index = self.get_index(id);
    self.data.swap_remove(index);
  }

  pub fn count(&self) -> usize {
    self.data.len()
  }

  fn _get(&self, index: usize) -> Option<&Document> {
    self.data.get(index)
  }

  fn get_index(&self, id: u32) -> usize {
    let id = DataType::Id(id);
    self.data.iter().position(|x| x.get(ID).unwrap() == &id).unwrap()
  }

  pub fn getAll(&self) -> &Vec<Document> {
    &self.data
    
   }

  pub fn get(&self, id: u32) -> Option<&Document> {
    let id = DataType::Id(id);
    self.data.iter().find(|&x| x.get(ID).unwrap() == &id)
  }

  pub fn remove(&mut self, index: usize) -> Document {
    self.data.remove(index)
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

