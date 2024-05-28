use std::fs;

struct Fileystem {
  fileName: &str
}

impl Fileystem {
  pub fn new(filename: &str) -> Self {
    Fileystem {
      fileName: filename
    }
  }

}