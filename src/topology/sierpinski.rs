use std::collections::HashSet;

#[derive(Debug)]
pub struct Shard {
    pub id: u64,
    pub level: u32,
    pub neighbors: HashSet<u64>,
}

impl Shard {
    pub fn new(id: u64, level: u32) -> Self {
        Self {
            id,
            level,
            neighbors: HashSet::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: u64) {
        self.neighbors.insert(neighbor_id);
    }
}

pub fn sierpinski(order: u32, current_id: &mut u64, current_level: u32) -> Vec<Shard> {
    if order == 0 {
        let shard = Shard::new(*current_id, current_level);
        *current_id += 1;
        return vec![shard];
    }

    let top = sierpinski(order - 1, current_id, current_level + 1);
    let left = sierpinski(order - 1, current_id, current_level + 1);
    let right = sierpinski(order - 1, current_id, current_level + 1);

    let mut all_shards = Vec::new();
    all_shards.extend(top);
    all_shards.extend(left);
    all_shards.extend(right);

    for shard in &mut all_shards {
        for neighbor in &all_shards {
            if shard.id != neighbor.id {
                shard.add_neighbor(neighbor.id);
            }
        }
    }

    all_shards
}
