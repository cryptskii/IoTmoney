use blake3::Hasher;
use dashmap::DashMap;
use serde::{Serialize, Deserialize};
use serde_cbor::{to_vec, from_slice};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    key: Option<Vec<u8>>,
    value: RwLock<Option<Vec<u8>>>,
    children: HashMap<u8, Arc<RwLock<Node>>>,
    epoch: u64,
}

impl Node {
    fn new(key: Option<Vec<u8>>, epoch: u64) -> Self {
        Self {
            key,
            value: RwLock::new(None),
            children: HashMap::new(),
            epoch,
        }
    }

    fn insert(&self, key: Vec<u8>, value: Vec<u8>, epoch: u64) {
        if key.is_empty() {
            let mut self_mut = self.value.write().unwrap();
            *self_mut = Some(value);
        } else {
            let idx = key[0];
            let child_key = key[1..].to_vec();
            let child_node = self.children.entry(idx).or_insert_with(|| {
                Arc::new(RwLock::new(Node::new(Some(child_key.clone()), epoch)))
            }).clone();
            child_node.write().unwrap().insert(child_key, value, epoch);
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        if key.is_empty() {
            self.value.read().unwrap().clone()
        } else {
            let idx = key[0];
            if let Some(child_node) = self.children.get(&idx) {
                child_node.read().unwrap().get(&key[1..])
            } else {
                None
            }
        }
    }

    fn hash(&self) -> blake3::Hash {
        let mut hasher = Hasher::new();
        if let Some(key) = &self.key {
            hasher.update(key);
        }
        if let Some(value) = &*self.value.read().unwrap() {
            hasher.update(value);
        }
        for child in self.children.iter() {
            hasher.update(&[*child.key()]);
            hasher.update(child.value().read().unwrap().hash().as_bytes());
        }
        hasher.finalize()
    }

    fn serialize(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        to_vec(self)
    }

    fn deserialize(data: &[u8]) -> Result<Self, serde_cbor::Error> {
        from_slice(data)
    }
}

#[derive(Debug, Clone)]
struct ConcurrentMerklePatriciaTrie {
    root: Arc<RwLock<Node>>,
}

impl ConcurrentMerklePatriciaTrie {
    fn new(epoch: u64) -> Self {
        Self {
            root: Arc::new(RwLock::new(Node::new(None, epoch))),
        }
    }

    fn insert(&self, key: Vec<u8>, value: Vec<u8>, epoch: u64) {
        self.root.write().unwrap().insert(key, value, epoch);
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.root.read().unwrap().get(key)
    }

    fn root_hash(&self) -> blake3::Hash {
        self.root.read().unwrap().hash()
    }
}

fn main() {
    let epoch = 1;
    let trie = Arc::new(ConcurrentMerklePatriciaTrie::new(epoch));

    let handles: Vec<_> = (0..10).map(|i| {
        let trie_clone = Arc::clone(&trie);
        std::thread::spawn(move || {
            let key = format!("key{}", i).into_bytes();
            let value = format!("value{}", i).into_bytes();
            trie_clone.insert(key, value, epoch);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let key = b"key5".to_vec();
    if let Some(value) = trie.get(&key) {
        println!("Retrieved: {:?}", String::from_utf8(value).unwrap());
    } else {
        println!("Key not found");
    }

    println!("Root Hash: {:?}", trie.root_hash());
}
