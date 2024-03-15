struct User {
    id: u32,
    username: String,
    password: String
}

impl User {
    pub fn new(username: String, password: String) -> User {
      let id = 0; //TODO: generate uuid
        User {
            id,
            username,
            password
        }
    }
}


