// Written by Alberto Ruiz 2024-03-07
// This is the main engine of the server, it will process the requests and return the responses
// 
// The engine will have a MEMOdb instance to store the data

use std::collections::HashMap;

use crate::memodb::data_type::DataType;
use crate::{doc, memodb::MEMOdb};
use crate::memodb::collection::{Document, DocumentJson};
use crate::hteapot::{HteaPot, HttpMethod, HttpRequest};
use crate::hteapot::HttpStatus;


pub struct Engine {
  db: MEMOdb

}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      db: MEMOdb::new()
    }
  }

  //TODO: remove this function, test only
  pub fn init_mock_data(&mut self) {
    let _ = self.db.create_collection("users".to_string());
    let collection = self.db.get_collection("users".to_string()).unwrap();
    collection.add(doc!{"name" => "John", "age" => 30});
    collection.add(doc!{"name" => "Jane", "age" => 25});
    collection.add(doc!{"name" => "Doe", "age" => 40});
  }

  //wrapper for MEMOdb functions

  fn get_collection_list(&self) -> String {
    let collections = self.db.get_collection_list();
    let list = format!("{:?}", collections);
    HteaPot::response_maker(HttpStatus::OK, &list)
  }

  fn get_document_by_id(&mut self, collection_name: &str, id: u32) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
            let document = collection.get(id);
            match document {
                Some(document) => {
                    let result = document.to_json();
                    HteaPot::response_maker(HttpStatus::OK, &result)
                }
                None => {
                    HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
                }
            }
        }
        None => {
            HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }

  fn get_all_documents(&mut self, collection_name: &str) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
            let mut body = String::from("[");
            let documents = collection.get_all();
            for document in documents {
                let result = document.to_json();
                body.push_str(&result);
                body.push_str(",");
            }
            body.pop();
            body.push_str("]");
            HteaPot::response_maker(HttpStatus::OK, &body)
        }
        None => {
            HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
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
            HteaPot::response_maker(HttpStatus::OK, &body)
        }
        None => {
            HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }

  fn delete_collection(&mut self, collection_name: &str) -> String {
    let collection = self.db.remove_collection(collection_name.to_string());
    let result = format!("{{\"collection\": \"{}\"}}", collection.name);
    HteaPot::response_maker(HttpStatus::OK, &result)
  }

  fn delete_document(&mut self, collection_name: &str, id: u32) -> String {
    let collection = self.db.get_collection(collection_name.to_string());
    match collection {
        Some(collection) => {
            collection.rm(id);
            HteaPot::response_maker(HttpStatus::OK, "OK")
        }
        None => {
            HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }


  //process the request and return the response
  pub fn process(&mut self, request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => {
            //path /collection_name/id
            let path = request.path.split("/").collect::<Vec<&str>>();
            println!("path: {:?}", path);
            if path.len() <= 2 {
                //list all collections
                return self.get_collection_list()
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
                    let id = path[2].parse::<u32>();
                    match id {
                        Ok(id) => {
                            self.get_document_by_id(collection_name, id)
                        }
                        Err(_) => {
                            HteaPot::response_maker(HttpStatus::BadRequest, "Bad Request")
                        }
                    }
                }
            }
        }
        HttpMethod::POST => {
            //path /collection_name OR /
            let path = request.path.split("/").collect::<Vec<&str>>();
            if path.len() == 2 {
                let collection_name = path[1];
                let result = self.db.create_collection(collection_name.to_string());
                match result {
                    Ok(_) => {
                        HteaPot::response_maker(HttpStatus::Created, "Created")
                    }, 
                    Err(_) => {
                        HteaPot::response_maker(HttpStatus::NotModified, "Collection already exists")
                    }
                }
            } else {
                let collection_name = path[1];
                let collection = self.db.get_collection(collection_name.to_string());
                match collection {
                    Some(collection) => {
                        println!("Request body: {}", request.body);
                        let document: Document = DocumentJson::from_json(&request.body);
                        let id = collection.add(document);
                        let result = format!("{{\"id\":{}}}", id);
                        HteaPot::response_maker(HttpStatus::Created, &result)
                    }
                    None => {
                        HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
                    }
                }
            }
        }
        HttpMethod::DELETE => {
            //path /collection_name OR /collection_name/id
            let path = request.path.split("/").collect::<Vec<&str>>();
            let collection_name = path[1];
            if path.len() == 2 {
                let confirmation = request.headers.get("amisure");
                match confirmation {
                    Some(confirmation) => {
                        if confirmation == "yes" {
                            self.delete_collection(collection_name)
                        } else {
                            HteaPot::response_maker(HttpStatus::Unauthorized, "add amisure header with value yes to confirm")
                        }
                    }
                    None => {
                        HteaPot::response_maker(HttpStatus::BadRequest, "Bad Request")
                    }
                }
            } else {
                let id = path[2].parse::<u32>().unwrap();
                self.delete_document(collection_name, id)
            }
        }
        _ => {
            HteaPot::response_maker(HttpStatus::NotImplemented, "Not Implemented")
        }
    }
  }
}