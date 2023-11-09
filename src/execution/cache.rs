use lru::LruCache;
use parking_lot::RwLock;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;

const NUM_SHARDS: usize = 10;

#[derive(Debug, Clone)]
struct Node {
    // Example fields
    data: Vec<u8>,
    // ... other fields ...
}

#[derive(Debug, Clone)]
struct Signature {
    // Example fields
    signature_data: Vec<u8>,
    // ... other fields ...
}

struct CacheEntry<T> {
    value: Arc<T>,
    expiry: Instant,
}

struct TimedCache<T> {
    shards: Vec<Arc<RwLock<LruCache<Vec<u8>, CacheEntry<T>>>>>,
    ttl: Duration,
}

impl<T> TimedCache<T> {
    fn new(ttl: Duration) -> Self {
        let mut shards = Vec::with_capacity(NUM_SHARDS);
        let cache_capacity = NonZeroUsize::new(5000).expect("Cache capacity must be non-zero");
        for _ in 0..NUM_SHARDS {
            shards.push(Arc::new(RwLock::new(LruCache::new(cache_capacity))));
        }
        Self { shards, ttl }
    }

    fn get(&self, key: &[u8]) -> Option<Arc<T>> {
        let shard = self.get_shard(key);
        let mut cache = shard.write();
        if let Some(entry) = cache.get_mut(key) {
            if entry.expiry >= Instant::now() {
                return Some(entry.value.clone());
            }
        }
        None
    }

    fn put(&self, key: Vec<u8>, value: Arc<T>) {
        let expiry = Instant::now() + self.ttl;
        let entry = CacheEntry { value, expiry };

        let shard = self.get_shard(&key);
        let mut cache = shard.write();
        cache.put(key, entry);
    }

    fn get_shard(&self, key: &[u8]) -> &Arc<RwLock<LruCache<Vec<u8>, CacheEntry<T>>>> {
        let idx = self.hash(key) % NUM_SHARDS;
        &self.shards[idx]
    }

    fn hash(&self, key: &[u8]) -> usize {
        // Simple hash function; replace with a better one if needed
        key.iter().fold(0, |acc, &byte| acc.wrapping_add(byte as usize))
    }
}

static TRIE_CACHE: Lazy<Arc<TimedCache<Node>>> = Lazy::new(|| {
    Arc::new(TimedCache::new(Duration::from_secs(10)))
});

static SIG_CACHE: Lazy<Arc<TimedCache<Signature>>> = Lazy::new(|| {
    Arc::new(TimedCache::new(Duration::from_secs(10)))
});

fn main() {
    // Example usage
    let key = vec![1, 2, 3];
    let node = Arc::new(Node { data: vec![4, 5, 6] });
    let signature = Arc::new(Signature { signature_data: vec![7, 8, 9] });

    // Concurrent read for Node
    {
        let trie_read = TRIE_CACHE.get(&key);
        if let Some(node) = trie_read {
            println!("Cached Node: {:?}", node);
        } else {
            println!("Node not found in cache");
        }
    }

    // Concurrent write for Node
    {
        TRIE_CACHE.put(key.clone(), node.clone());
        println!("Node added to cache");
    }

    // Concurrent read for Signature
    {
        let sig_read = SIG_CACHE.get(&key);
        if let Some(signature) = sig_read {
            println!("Cached Signature: {:?}", signature);
        } else {
            println!("Signature not found in cache");
        }
    }

    // Concurrent write for Signature
    {
        SIG_CACHE.put(key, signature.clone());
        println!("Signature added to cache");
    }
}

