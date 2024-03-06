use std::collections::HashMap;

use crate::memodb::dataType::DataType;
use crate::{doc, memodb::MEMOdb, response_maker, HttpMethod, HttpRequest, HttpStatus};
use crate::memodb::collection::{Document, DocumentJson};

pub struct Engine {
  db: MEMOdb

}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      db: MEMOdb::new()
    }
  }
  pub fn init_mock_data(&mut self) {
    self.db.create_collection("users".to_string());
    let collection = self.db.get_collection("users".to_string()).unwrap();
    collection.add(doc!{"name" => "John", "age" => 30});
    collection.add(doc!{"name" => "Jane", "age" => 25});
    collection.add(doc!{"name" => "Doe", "age" => 40});
  }

  fn get_collection_list(&self) -> String {
    let collections = self.db.get_collection_list();
    let list = collections.join("\n");
    list
  }

  fn get_document_by_id(&mut self, collection_name: &str, id: u32) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
            let document = collection.get(id);
            match document {
                Some(document) => {
                    let result = document.to_json();
                    response_maker(HttpStatus::OK, &result)
                }
                None => {
                    response_maker(HttpStatus::NotFound, "Not Found")
                }
            }
        }
        None => {
            response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }

  fn get_all_documents(&mut self, collection_name: &str) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
            let mut body = String::from("[");
            let documents = collection.getAll();
            for document in documents {
                let result = document.to_json();
                body.push_str(&result);
                body.push_str(",");
            }
            body.pop();
            body.push_str("]");
            response_maker(HttpStatus::OK, &body)
        }
        None => {
            response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }

  fn find(&mut self, collection_name: &str, args: HashMap<String,String>) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
        let mut body = String::from("[");
        let args: HashMap<String, DataType> = args.iter().map(|(k, v)| (k.to_string(), DataType::from_json(v))).collect();
        let documents: Vec<&Document> = collection.find(args);
            for document in documents {
                let result = document.to_json();
                body.push_str(&result);
                body.push_str(",");
            }
            if body.ends_with(',') {body.pop();}
            body.push_str("]");
            response_maker(HttpStatus::OK, &body)
        }
        None => {
            response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }


  pub fn process(&mut self, request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => {
            //path /collection_name/id
            let path = request.path.split("/").collect::<Vec<&str>>();
            println!("path: {:?}", path);
            if path.len() != 3 {
                //list all collections
                let list = self.get_collection_list();
                return response_maker(HttpStatus::OK, &list);
            }
            let collection_name = path[1];
            match path[2] {
                "all" => {
                    self.get_all_documents(collection_name)
                }
                "find" => {
                    println!("args: {:?}", request.args);
                    self.find(collection_name, request.args)
                }
                _ => {
                    let id = path[2].parse::<u32>().unwrap();
                    self.get_document_by_id(collection_name, id)
                }
            }
        }
        HttpMethod::POST => {
            //path /collection_name
            let path = request.path.split("/").collect::<Vec<&str>>();
            let collection_name = path[1];
            let collection = self.db.get_collection(collection_name.to_string());
            match collection {
                Some(collection) => {
                    println!("Request body: {}", request.body);
                    let document: Document = DocumentJson::from_json(&request.body);
                    let id = collection.add(document);
                    let result = format!("{{\"id\":{}}}", id);
                    response_maker(HttpStatus::Created, &result)
                }
                None => {
                    response_maker(HttpStatus::NotFound, "Not Found")
                }
            }
        }
        _ => {
            response_maker(HttpStatus::NotImplemented, "Not Implemented")
        }
    }
  }
}