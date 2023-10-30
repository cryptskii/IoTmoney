use std::collections::HashMap;
use rand::Rng;

struct TopologyManager {
    shards: HashMap<u64, Shard>,
}

impl TopologyManager {
    fn new() -> Self {
        Self {
           shards: HashMap::new(),
        }
    }

    fn add_shard(&mut self, shard: Shard) {
        self.shards.insert(shard.id, shard);
    }

    fn remove_shard(&mut self, shard_id: u64) {
        self.shards.remove(&shard_id);
    }

    fn connect_shards(&mut self, shard1_id: u64, shard2_id: u64) {
        if let Some(shard1) = self.shards.get_mut(&shard1_id) {
            shard1.connect(shard2_id);
        }
        if let Some(shard2) = self.shards.get_mut(&shard2_id) {
            shard2.connect(shard1_id);
        }
    }

    fn disconnect_shards(&mut self, shard1_id: u64, shard2_id: u64) {
        if let Some(shard1) = self.shards.get_mut(&shard1_id) {
            shard1.disconnect(shard2_id);
        }
        if let Some(shard2) = self.shards.get_mut(&shard2_id) {
            shard2.disconnect(shard1_id);
        }
    }

    fn apply_sierpinski(&mut self, shard_id: u64, levels: u32) {
        if levels == 0 || !self.shards.contains_key(&shard_id) {
            return;
        }

        let mut rng = rand::thread_rng();
        let (child1_id, child2_id, child3_id): (u64, u64, u64) = (rng.gen(), rng.gen(), rng.gen());

        // Create and add new sub-shards
        let state = ShardState { /* Initialize state */ };
        self.add_shard(Shard::new(child1_id, state.clone()));
        self.add_shard(Shard::new(child2_id, state.clone()));
        self.add_shard(Shard::new(child3_id, state));

        // Connect new sub-shards in a triangular pattern
        self.connect_shards(child1_id, child2_id);
        self.connect_shards(child2_id, child3_id);
        self.connect_shards(child1_id, child3_id);

        // Recursively apply Sierpinski pattern to sub-shards
        self.apply_sierpinski(child1_id, levels - 1);
        self.apply_sierpinski(child2_id, levels - 1);
        self.apply_sierpinski(child3_id, levels - 1);
    }
}
