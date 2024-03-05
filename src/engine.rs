use crate::{doc, memodb::MEMOdb, response_maker, HttpMethod, HttpRequest, HttpStatus};
use crate::memodb::collection::{DocumentJson, Document};

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
  pub fn process(&mut self, request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => {
            //path /collection_name/id
            let path = request.path.split("/").collect::<Vec<&str>>();
            if path.len() != 3 {
                //list all collections
                let collections = self.db.get_collection_list();
                let list = collections.join("\n");
                return response_maker(HttpStatus::OK, &list);
            }
            let collection_name = path[1];
            let id = path[2].parse::<u32>().unwrap();
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