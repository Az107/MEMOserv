// Written by Alberto Ruiz 2024-03-08
// MEMOdb is a in-memory database, 
// it will store the data in memory and provide a simple API to interact with it
//
// The MEMOdb will have a collection of documents, each document will be a HashMap<String, DataType>
    

pub mod collection;
pub mod data_type;
mod finder;
use collection::Collection;

pub struct MEMOdb {
    pub version: &'static str,
    collections: Vec<Collection>,
}

impl MEMOdb {
    pub fn new() -> Self {
        MEMOdb {
            version: "0.1.5",
            collections: Vec::new(),
        }
    }

    pub fn create_collection(&mut self, name: String) -> Result<(), &str>{
        //check if collection exists
        if self.collections.iter().any(|x| x.name == name) {
            Err("Collection already exists")
        } else {
            let collection = Collection::new(name);
            self.collections.push(collection);
            Ok(())
        }
    }

    pub fn get_collection(&mut self, name: String) -> Option<&mut Collection> {
        //return a mutable reference to collection
        self.collections.iter_mut().find(|x| x.name == name)
    }

    pub fn get_collection_list(&self) -> Vec<String> {
        let mut collection_list: Vec<String> = Vec::new();
        for collection in self.collections.iter() {
            collection_list.push(collection.name.clone());
        }
        collection_list
    }

    pub fn remove_collection(&mut self, name: String) -> Collection {
        let index = self
            .collections
            .iter()
            .position(|x| x.name == name)
            .unwrap();
        self.collections.remove(index)
    }

}


//TEST
#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::doc;
    use crate::memodb::collection::Document;
    use crate::memodb::collection::DocumentStruct;
    use crate::memodb::collection::DocumentJson;

    struct User {
        name: String,
        age: i32,
    }

    impl DocumentStruct for User {
        fn to_document(&self) -> Document {
            doc!{"name" => self.name.clone(), "age" => self.age}
        }

        fn from_document(document: &Document) -> Self {
            User {
                name: document.get("name").unwrap().to_string(),
                age: document.get("age").unwrap().to_number()
            }
        }
        
    }


    #[test]
    fn test_memodb() {
        let mut memodb = crate::memodb::MEMOdb::new();
        let r1 = memodb.create_collection("users".to_string()).is_ok();
        let r2 = memodb.create_collection("posts".to_string()).is_ok();
        assert!(r1);
        assert!(r2);
        assert_eq!(memodb.collections.len(), 2);
        assert_eq!(memodb.collections[0].name, "users");
        assert_eq!(memodb.collections[1].name, "posts");
        assert_eq!(memodb.get_collection("users".to_string()).unwrap().name, "users");
        assert_eq!(memodb.get_collection("posts".to_string()).unwrap().name, "posts");
        assert_eq!(memodb.get_collection_list().len(), 2);
        assert_eq!(memodb.remove_collection("users".to_string()).name, "users");
        assert_eq!(memodb.collections.len(), 1);
        assert_eq!(memodb.remove_collection("posts".to_string()).name, "posts");
        assert_eq!(memodb.collections.len(), 0);
    }

    #[test]
    fn add_document() {
        let mut memodb = crate::memodb::MEMOdb::new();
        let _ = memodb.create_collection("users".to_string());
        let collection = memodb.get_collection("users".to_string()).unwrap();
        collection.add(doc!{"name" => "John", "age" => 30});
        collection.add(doc!{"name" => "Jane", "age" => 25});
        assert_eq!(collection.count(), 2);
        let document = collection.get(1).unwrap();
        let user = User::from_document(document);
        assert_eq!(user.name, "John");
    } 

    #[test]
    fn add_document_from_struct() {
        let mut memodb = crate::memodb::MEMOdb::new();
        let _ = memodb.create_collection("users".to_string());
        let collection = memodb.get_collection("users".to_string()).unwrap();
        let user = User {
            name: "John".to_string(),
            age: 30,
        };
        let id = collection.add(user.to_document());
        assert_eq!(collection.count(), 1);
        let document = collection.get(id).unwrap();
        document.to_json();
        let user = User::from_document(document);
        assert_eq!(user.name, "John");
    }


    #[test]
    fn write_benchmark() {
        const VALUE : u32 = 5;
        // create a repetition value pow 10 to VALUE
        let repetition = 10i32.pow(VALUE);
        let start = Instant::now();
        let mut memodb = crate::memodb::MEMOdb::new();
        let _ = memodb.create_collection("test".to_string());
        let collection = memodb.get_collection("test".to_string()).unwrap();
        for i in 0..repetition {
            collection.add(doc!{"name" => i});
        }
        let duration = start.elapsed();
        println!("🟦 Time elapsed in writing {} repetitions: {:?}", repetition, duration);
        assert!(duration.as_millis() < 150);

    }
}