use std::collections::HashMap;
use rand::Rng;

struct Shard {
    id: u64,
    load: f64,
    state: ShardState,
    connections: Vec<u64>,
}

impl Shard {
    fn new(id: u64, state: ShardState) -> Self {
        Self {
            id,
            load: 0.0,
            state,
            connections: Vec::new(),
        }
    }

    fn connect(&mut self, other_shard_id: u64) {
        self.connections.push(other_shard_id);
    }

    fn disconnect(&mut self, other_shard_id: u64) {
        self.connections.retain(|&id| id != other_shard_id);
    }

    fn update_load(&mut self, load: f64) {
        self.load = load;
    }
}

struct ShardState {
    // Define the state attributes for each shard
    // Example: data: HashMap<String, String>,
}
