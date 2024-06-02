// Written by Alberto Ruiz 2024-03-08
// The data type module will provide the data types for the MEMOdb
// this will be store several types of data, like text, numbers, dates, arrays and documents
//
// The data type will be used to store the data in the documents
use uuid::Uuid;
use super::collection::{Document, DocumentJson};

#[derive(PartialEq)]
pub enum DataType {
  Id(Uuid),
  Text(String),
  Number(i32),
  Boolean(bool),
  Array(Vec<DataType>),
  Document(Document),
}

impl DataType {
  pub fn get_type(&self) -> &str {
    match self {
      DataType::Id(_) => "id",
      DataType::Text(_) => "text",
      DataType::Number(_) => "number",
      DataType::Boolean(_) => "boolean",
      DataType::Array(_) => "array",
      DataType::Document(_) => "document",
    }
  }

  #[deprecated]
  pub fn to_json(&self) -> String {
    match self {
      DataType::Id(id) => format!("\"{}\"", id.to_string()),
      DataType::Text(text) => format!("\"{}\"", text),
      DataType::Number(number) => number.to_string(),
      DataType::Boolean(boolean) => boolean.to_string(),
      DataType::Array(array) => {
        let mut json = String::from("[");
        for value in array {
          json.push_str(&value.to_json());
          json.push_str(",");
        }
        json.pop();
        json.push(']');
        json
      }
      DataType::Document(document) => {
        let mut json = String::from("{");
        for (key, value) in document {
          json.push_str(&format!("\"{}\":{},", key, value.to_json()));
        }
        json.pop();
        json.push('}');
        json
      }
    }
  }

  #[deprecated]
  pub fn from_json(json: &str) -> DataType {
    let json = json.trim();
    if json.starts_with('[') {
      let mut array = Vec::new();
      let json = &json[1..json.len() - 1];
      let json = json.split(',');
      for value in json {
        array.push(DataType::from_json(value));
      }
      DataType::Array(array)
    } else if json.starts_with('{') {
      let document = Document::from_json(json);
      if document.is_err() {
        return DataType::Document(Document::new());
      } else {
        return DataType::Document(document.unwrap());
      }
    } else if json.starts_with('\"') {
      DataType::Text(json[1..json.len() - 1].to_string())
    } else if json == "true" {
      DataType::Boolean(true)
    } else if json == "false" {
      DataType::Boolean(false)
    } else {
      match json.parse::<i32>() {
        Ok(number) => DataType::Number(number),
        Err(_) => DataType::Text(json.to_string()),
      }
    }
  }
  //add into 
  pub fn to_id(&self) -> Uuid {
    match self {
      DataType::Id(id) => *id,
      _ => panic!("Not an ID"),
    }
  }
  pub fn to_text(&self) -> &String {
    match self {
      DataType::Text(text) => text,
      _ => panic!("Not a Text"),
    }
  }
  pub fn to_number(&self) -> i32 {
    match self {
      DataType::Number(number) => *number,
      _ => panic!("Not a Number"),
    }
  }
  pub fn to_boolean(&self) -> bool {
    match self {
      DataType::Boolean(boolean) => *boolean,
      _ => panic!("Not a Boolean"),
    }
  }
  pub fn to_array(&self) -> &Vec<DataType> {
    match self {
      DataType::Array(array) => array,
      _ => panic!("Not an Array"),
    }
  }
  pub fn to_document(&self) -> &Document {
    match self {
      DataType::Document(document) => document,
      _ => panic!("Not a Document"),
    }
  }
  
}

impl ToString for DataType {
  fn to_string(&self) -> String {
    match self {
      DataType::Id(id) => id.to_string(),
      DataType::Text(text) => text.to_string(),
      DataType::Number(number) => number.to_string(),
      DataType::Boolean(boolean) => boolean.to_string(),
      DataType::Array(array) => {
        let mut result = String::new();
        for value in array {
          result.push_str(&value.to_string());
          result.push_str(", ");
        }
        result
      }
      DataType::Document(document) => {
        let mut result = String::new();
        for (key, value) in document {
          result.push_str(&key);
          result.push_str(": ");
          result.push_str(&value.to_string());
          result.push_str(", ");
        }
        result
      }
    }
  }
}


impl From<Uuid> for DataType {
  fn from(value: Uuid) -> Self {
    DataType::Id(value)
  }
}

impl From<String> for DataType {
  fn from(value: String) -> Self {
    DataType::Text(value)
  }
}

impl From<&str> for DataType {
  fn from(value: &str) -> Self {
    DataType::Text(value.to_string())
  }
}

impl From<i32> for DataType {
  fn from(value: i32) -> Self {
    DataType::Number(value)
  }
}

impl From<bool> for DataType {
  fn from(value: bool) -> Self {
    DataType::Boolean(value)
  }
}

impl From<Vec<DataType>> for DataType {
  fn from(value: Vec<DataType>) -> Self {
    DataType::Array(value)
  }
}

impl From<Document> for DataType {
  fn from(value: Document) -> Self {
    DataType::Document(value)
  }
}

//impl clone
impl Clone for DataType {
  fn clone(&self) -> Self {
    match self {
      DataType::Id(id) => DataType::Id(*id),
      DataType::Text(text) => DataType::Text(text.clone()),
      DataType::Number(number) => DataType::Number(*number),
      DataType::Boolean(boolean) => DataType::Boolean(*boolean),
      DataType::Array(array) => DataType::Array(array.clone()),
      DataType::Document(document) => DataType::Document(document.clone()),
    }
  }
}