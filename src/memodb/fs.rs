use std::fs;

use super::collection::{self, Collection, DocumentJson};
use serde_json::Value;


struct Fileystem {
  filename: String
}

impl Fileystem {
  pub fn new(filename: &str) -> Self {
    Fileystem {
      filename: filename.to_string()
    }
  }


  pub fn load(&self) -> Result<Collection,&str> {
    let contents = fs::read_to_string(self.filename.as_str());
    if contents.is_err() {
      return Err("Error reading file");
    }
    let contents = contents.unwrap();
    let mut collections: Vec<Collection> = Vec::new();

    return Ok(collections);
  }

  pub fn save_all(&self,collections: Vec<Collection>) -> Result<(),&str> {

    for collection in collections {
      let json_collection = collection.to_json();
    }
    fs::write(self.filename, json_collection);
    Ok(())
  }

}