struct JSONparsed {
    key: String,
    value: Option<String>,
    children: Vec<JSONparsed>

}

struct JSONparser { 
  current_node: Option<JSONparsed>,
  bracket_count: u32,
}

impl JSONparser {
    pub fn new() -> JSONparser {
        JSONparser {
            current_key: None,
            current_value: None,
            bracket_count: 0,
        }
    }

    pub fn parse(&self, json: &str) -> Result<JSONparsed> {
        self.current_node = Some(JSONparsed {
            key: "root".to_string(),
            value: None,
            children: Vec::new(),
        });

        for c in json.chars() {
            match c {
                '{' => {
                    self.bracket_count += 1;
                }
                '}' => {
                    self.bracket_count -= 1;
                }
                '"' => {
                    if self.current_key.is_none() {
                        self.current_key = Some(String::new());
                    } else if self.current_value.is_none() {
                        self.current_value = Some(String::new());
                    }
                }
                _ => {
                    if self.current_key.is_some() {
                        self.current_key.as_mut().unwrap().push(c);
                    } else if self.current_value.is_some() {
                        self.current_value.as_mut().unwrap().push(c);
                    }
                }
            }
        }
    }
}