use sled::Db;

pub struct StorageDatabase {
  db: Db
}

impl StorageDatabase {

  pub fn new(path: &str) -> Self {
    let db = sled::open(path).unwrap();
    StorageDatabase { db }
  }

  pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
    self.db.get(key).unwrap().map(|v| v.to_vec())
  }

  pub fn insert(&mut self, key: &[u8], value: &[u8]) {
    self.db.insert(key, value).unwrap();
  }

}