use blake3::{hash, Hasher};

pub fn hash_data(data: &[u8]) -> [u8; 32] {
  let mut hasher = Hasher::new();
  hasher.update(data);
  hash(data)
}