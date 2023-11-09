use std::collections::{HashSet};

#[derive(Clone, Debug)]
struct Node {
    id: u32,
    // Other node properties
}

#[derive(Clone, Debug)]
struct Shard {
    id: u32,
    nodes: Vec<Node>,
}

struct ShardManager {
    shards: Vec<Shard>,
    config: ShardConfig,
}

struct ShardConfig {
    min_nodes: u32,
    max_nodes: u32,
}

impl ShardManager {
    fn balance_shards(&mut self) {
    let mut new_shards = Vec::new();
    let mut shards_to_remove = HashSet::new();

    for i in 0..self.shards.len() {
        if shards_to_remove.contains(&self.shards[i].id) {
            continue;
        }

        if self.should_split(&self.shards[i]) {
            let shard_clone = self.shards[i].clone();
            let (shard_1, shard_2) = self.split_shard(&shard_clone);
            new_shards.push(shard_1);
            new_shards.push(shard_2);
        } else if self.should_merge(&self.shards[i]) {
            if let Some(other_shard) = self.find_shard_to_merge_with(&self.shards[i]) {
                if !shards_to_remove.contains(&other_shard.id) &&
                (self.shards[i].nodes.len() + other_shard.nodes.len()) as u32 <= self.config.max_nodes {
                    let merged_shard = self.merge_shards(&self.shards[i], other_shard);
                    new_shards.push(merged_shard);
                    shards_to_remove.insert(other_shard.id);
                } else {
                    new_shards.push(self.shards[i].clone());
                }
            } else {
                new_shards.push(self.shards[i].clone());
            }
        } else {
            new_shards.push(self.shards[i].clone());
        }
    }

    self.shards = new_shards;
    self.shards.retain(|shard| !shards_to_remove.contains(&shard.id));
}


    fn should_split(&self, shard: &Shard) -> bool {
        (shard.nodes.len() as u32) > self.config.max_nodes
    }

    fn should_merge(&self, shard: &Shard) -> bool {
        (shard.nodes.len() as u32) < self.config.min_nodes
    }

    fn split_shard(&mut self, shard: &Shard) -> (Shard, Shard) {
        let mid = shard.nodes.len() / 2;
        let shard_1 = Shard {
            id: self.shards.len() as u32 + 1,
            nodes: shard.nodes[..mid].to_vec(),
        };
        let shard_2 = Shard {
            id: self.shards.len() as u32 + 2,
            nodes: shard.nodes[mid..].to_vec(),
        };
        (shard_1, shard_2)
    }

    fn merge_shards(&self, shard1: &Shard, shard2: &Shard) -> Shard {
        let mut merged_shard = shard1.clone();
        merged_shard.nodes.extend_from_slice(&shard2.nodes);
        merged_shard
    }

    fn find_shard_to_merge_with(&self, shard: &Shard) -> Option<&Shard> {
        self.shards.iter()
            .filter(|&other_shard| other_shard.id != shard.id)
            .filter(|&other_shard| (shard.nodes.len() + other_shard.nodes.len()) as u32 <= self.config.max_nodes)
            .min_by_key(|&other_shard| other_shard.nodes.len())
    }
}


   
fn main() {
    // Create shard configuration
    let config = ShardConfig {
        min_nodes: 2,
        max_nodes: 5,
    };

    // Create a ShardManager with the configuration
    let mut shard_manager = ShardManager {
        shards: Vec::new(),
        config,
    };

    // Create and add some shards with nodes
    for i in 0..3 {
        let mut shard = Shard {
            id: i,
            nodes: Vec::new(),
        };
        for j in 0..(i + 3) {
            shard.nodes.push(Node { id: j });
        }
        shard_manager.shards.push(shard);
    }

    // Display initial state
    println!("Initial state:");
    display_shard_manager(&shard_manager);

    // Perform shard balancing
    shard_manager.balance_shards();

    // Display state after balancing
    println!("\nState after balancing:");
    display_shard_manager(&shard_manager);
}

fn display_shard_manager(shard_manager: &ShardManager) {
    for shard in &shard_manager.shards {
        println!("Shard ID: {}", shard.id);
        for node in &shard.nodes {
            println!("  Node ID: {}", node.id);
        }
    }
}
