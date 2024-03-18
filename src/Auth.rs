
struct User {
    id: Uuid,
    username: String,
    password: String
}

impl User {
    pub fn new(username: String, password: String) -> User {
      let id = Uuid::new_v4();
        User {
            id,
            username,
            password
        }
    }
}


