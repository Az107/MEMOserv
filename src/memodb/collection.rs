// Writen by Alberto Ruiz 2024-03-08
// The collection module will provide the collection of documents for the MEMOdb
// The collection will store the documents in memory and provide a simple API to interact with them
// The Document will be a HashMap<String, DataType> 

use rayon::collections;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use super::data_type::DataType;
use serde_json::Value;

const ID: &str = "ID";

//create a trait based on HashMap<String,DataType>
// and impl especial methods for it
pub type Document = HashMap<String, DataType>;


pub trait DocumentStruct {
  fn to_document(&self) -> Document;
  fn from_document(document: &Document) -> Self;
}

pub trait DocumentJson: Sized {
  fn to_json(&self) -> String;
  fn to_json_value(&self) -> Value;
  fn from_json(json: &str) -> Result<Self ,&str>;
}

impl DocumentJson for Document {
  fn to_json(&self) -> String {
    
    self.to_json_value().to_string()
  }

  fn to_json_value(&self) -> Value {
    let mut json = serde_json::json!({});
    for (key, value) in self.iter() {
      match value {
        DataType::Id(id) => json[key] = serde_json::json!(id.to_string()),
        DataType::Text(text) => json[key] = serde_json::json!(text),
        DataType::Number(number) => json[key] = serde_json::json!(number),
        DataType::Boolean(boolean) => json[key] = serde_json::json!(boolean),
        _ => json[key] = serde_json::json!("()")
      }
    }
    json
  }

  fn from_json(json: &str) -> Result<Self, &str> {
      let v: Value = serde_json::from_str(json).unwrap();
      let mut document = Document::new();
      for (key, value) in v.as_object().unwrap() {
        let value: Value = value.clone();
        if key == ID {
          let value_is_string = value.is_string();
          let id = Uuid::parse_str(value.as_str().unwrap());
          if id.is_ok() && value_is_string {
            document.insert(key.to_string(), DataType::Id(id.unwrap()));
          } else {
            match value {
              Value::Number(n) => document.insert("id".to_string(), DataType::Number(n.as_i64().unwrap() as i32)),
              Value::String(s) => document.insert("id".to_string(), DataType::Text(s)),
              Value::Bool(b) => document.insert("id".to_string(), DataType::Boolean(b)),
              _ => document.insert("id".to_string(), DataType::Text("".to_string()))
            };
          }
        } else {
          match value {
            Value::Number(n) => document.insert(key.to_string(), DataType::Number(n.as_i64().unwrap() as i32)),
            Value::String(s) => document.insert(key.to_string(), DataType::Text(s)),
            Value::Bool(b) => document.insert(key.to_string(), DataType::Boolean(b)),
            _ => document.insert(key.to_string(), DataType::Text("".to_string()))
          };
        }
      }
  
    Ok(document)
  
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
  pub(crate) data: Vec<Document>,
  id_table: HashMap<Uuid, usize>,
  //b_tree: BNode
}

impl DocumentJson for Collection {
    fn to_json_value(&self) -> Value {
        let mut json = serde_json::json!({
          "name": self.name,
          "data": []
        });
        let mut jsondata = Vec::new();
        for document in self.data.iter() {
          jsondata.push(document.to_json_value())
        }
        json["data"] = serde_json::Value::Array(jsondata);
        println!("col is {} , {}",json.is_object(), json.to_string().as_str());
        json
    }

    fn to_json(&self) -> String {
        self.to_json_value().to_string()
    }

    fn from_json(json: &str) -> Result<Self,&str> {
      let v: Value = serde_json::from_str(json).unwrap();
      let obj =  v.as_object().unwrap();
      let name = obj.get("name");
      if name.is_none() {
        return Err("()");
      }
      let name = name.unwrap().to_string().replace("\"", "");
      let mut collection = Collection::new(name);
      let data = obj.get("data");
      if data.is_none() {
        return Err("()");
      }
      let data = data.unwrap().as_array();
      if data.is_none() {
        return Err("Error converting data as array");
      }
      let data = data.unwrap();
      for document in data {
        let document = document.to_string();
        let document = document.as_str();
        let doc = Document::from_json(document);
        if doc.is_err() {continue;}
        let doc = doc.unwrap();
        collection.add(doc);
      }
      return Ok(collection);
      
    }
}

impl Collection {
  pub fn new(name: String) -> Self {
    Collection {
      name: name,
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

  pub fn get_all(&self,limit: usize, offset: usize) -> Vec<Document> {
    if limit == 0 {
      return self.data.clone()
    }
    // let limit = limit+offset;
    // if limit > self.data.len() {
    //   return self.data[offset..].to_vec()
    // }
    self.data[offset..limit].to_vec()
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
      let is_id = value.to_string().parse::<Uuid>();
      if key == ID && is_id.is_ok() {
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

  fn slow_get(&mut self, id: Uuid) -> Option<&mut Document> {
    let id = DataType::Id(id);
    self.data.iter_mut().find(|x| x.get(ID).unwrap() == &id)


  }

  pub fn get(&mut self, id: Uuid) -> Option<&mut Document> {
    let index = self.id_table.get(&id);
    match index {
      Some(index) => self.data.get_mut(*index),
      None => self.slow_get(id)
    }
  }


  pub fn update_document(&mut self,id: Uuid, new_document: Document) -> Option<&Document> {
    let document = self.get(id).unwrap();
    for (key, val) in new_document.iter() {
      document.remove(key);
      document.insert(key.to_string(), val.clone());
    }
    return Some(document);
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
