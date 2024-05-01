// Written by Alberto Ruiz 2024-03-07
// This is the main engine of the server, it will process the requests and return the responses
// 
// The engine will have a MEMOdb instance to store the data

use std::collections::HashMap;
use uuid::{uuid, Uuid};

use crate::memodb::data_type::DataType;
use crate::{doc, memodb::MEMOdb};
use crate::memodb::collection::{self, Document, DocumentJson};
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

  fn get_document_by_id(&mut self, collection_name: String, id: Uuid) -> String {
    let collection = self.db.get_collection(collection_name);
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

  fn get_all_documents(&mut self, collection_name: String,limit: usize, offset: usize) -> String {
    let collection = self.db.get_collection(collection_name);
    match collection {
        Some(collection) => {
            let mut body = String::from("[");
            let documents = collection.get_all(limit, offset);
            for document in documents {
                let result = document.to_json();
                body.push_str(&result);
                body.push_str(",");
            }
            if body.ends_with(',') { body.pop(); }
            body.push_str("]");
            HteaPot::response_maker(HttpStatus::OK, &body)
        }
        None => {
            HteaPot::response_maker(HttpStatus::NotFound, "Not Found")
        }
    }
  }

  fn find(&mut self, collection_name: String, args: HashMap<String,String>) -> String {
    let collection = self.db.get_collection(collection_name);
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

  fn delete_collection(&mut self, collection_name: String) -> String {
    let collection = self.db.remove_collection(collection_name);
    let result = format!("{{\"collection\": \"{}\"}}", collection.name);
    HteaPot::response_maker(HttpStatus::OK, &result)
  }

  fn delete_document(&mut self, collection_name: String, document: String) -> String {
    let id = Uuid::parse_str(document.as_str());
    if id.is_err() {
        HteaPot::response_maker(HttpStatus::BadRequest, "Invalid id");
    }
    let id = id.unwrap();
    let collection = self.db.get_collection(collection_name);
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
    let mut path = request.path.split("/").collect::<Vec<&str>>();
    path.retain(|&x| x != "");
    let collection_name:Option<String> =  if path.len() >= 1 {Some(path[0].to_string())} else {None};
    let document_name: Option<String> =  if path.len() >= 2 {Some(path[1].to_string())} else {None};
    println!("PATH {:?}",path);
    println!("col {:?}", collection_name);
    println!("doc {:?}", document_name);


    match request.method {
        HttpMethod::GET => {
            // PATH /{collection_name}/{id}
            // 1. / -> list of collections
            // 2. /{collection_name} -> collection exists ? 200 : 404
            // 3. /{collection_name}/{id}
            //      a. id == uuid -> return document 
            //      b. id == all -> return all documents (should be paginted?)
            //      c. id == find?query -> return all documents that has one or      

            //path /
            if collection_name.is_none() {
                //list all collections
                return self.get_collection_list()
            }
            let collection_name = collection_name.unwrap();
            if document_name.is_none() {
                let r = self.db.get_collection(collection_name);
                return match r {
                    Some(_) => HteaPot::response_maker(HttpStatus::OK, ""),
                    None => HteaPot::response_maker(HttpStatus::NotFound, ""),
                };
            }
            let document_name = document_name.unwrap();
            let d = document_name.as_str();
            match d {
                "all" => {
                    let limit = request.args.get("limit").unwrap_or(&"0".to_string()).parse::<usize>().unwrap();
                    let offset = request.args.get("offset").unwrap_or(&"0".to_string()).parse::<usize>().unwrap();
                    self.get_all_documents(collection_name,limit, offset)
                }
                "find" => {
                    println!("args: {:?}", request.args);
                    self.find(collection_name, request.args)
                }
                _ => {
                    let id = document_name.parse::<Uuid>();
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
            if collection_name.is_none() {
                return HteaPot::response_maker(HttpStatus::BadRequest, "Bad request");
            }
            let collection_name = collection_name.unwrap();
            if document_name.is_none() {
                let result = self.db.create_collection(collection_name);
                match result {
                    Ok(_) => {
                        HteaPot::response_maker(HttpStatus::Created, "Created")
                    }, 
                    Err(_) => {
                        HteaPot::response_maker(HttpStatus::NotModified, "Collection already exists")
                    }
                }
            } else {
                let collection = self.db.get_collection(collection_name);
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
            // PATH /{collection_name}/{id}
            if collection_name.is_none() {
                return HteaPot::response_maker(HttpStatus::BadRequest, "Bad request");
            }
            let collection_name = collection_name.unwrap();
            if document_name.is_some() {
                let document = document_name.unwrap();
                return self.delete_document(collection_name, document)
            } else {
                let confirmation = request.headers.get("amisure");
                match confirmation {
                    Some(confirmation) => {
                        if confirmation == "yes" {
                            return self.delete_collection(collection_name);
                        } else {
                            return HteaPot::response_maker(HttpStatus::Unauthorized, "add amisure header with value yes to confirm");
                        }
                    }
                    None => {
                        return HteaPot::response_maker(HttpStatus::BadRequest, "Bad Request")
                    }
                }
            }
        }
        HttpMethod::PUT => {
            if collection_name.is_some() && document_name.is_some() {
                let collection_name  = collection_name.unwrap();
                let document_name = document_name.unwrap();
                let new_document: Document = DocumentJson::from_json(&request.body);
                let collection = self.db.get_collection(collection_name);
                if collection.is_none() {return HteaPot::response_maker(HttpStatus::NotFound, "Collection not found"); }
                let collection = collection.unwrap();
                let id = Uuid::parse_str(document_name.as_str());
                if id.is_err() {return HteaPot::response_maker(HttpStatus::BadRequest, "Bad request"); };
                let id = id.unwrap();
                collection.update_document(id, new_document);
                return HteaPot::response_maker(HttpStatus::OK, "Updated");

            } else {
                return HteaPot::response_maker(HttpStatus::BadRequest, "Bad request");
            }
        }
        _ => {
            HteaPot::response_maker(HttpStatus::NotImplemented, "Not Implemented")
        }
    }
  }
}