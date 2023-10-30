#[derive(Debug, Clone)]
struct Shard {
    id: usize,
    level: usize,
    neighbors: Vec<usize>,
}

impl Shard {
    fn new(id: usize, level: usize) -> Self {
        Shard {
            id,
            level,
            neighbors: Vec::new(),
        }
    }
}

fn create_sierpinski_topology(level: usize, parent: Shard, shards: &mut Vec<Shard>) {
    if level == 0 {
        return;
    }

    let base_id = shards.len();
    let child_level = level - 1;

    // Create the three child shards
    let mut shard1 = Shard::new(base_id, child_level);
    let mut shard2 = Shard::new(base_id + 1, child_level);
    let mut shard3 = Shard::new(base_id + 2, child_level);

    // Connect the child shards to each other
    shard1.neighbors.push(shard2.id);
    shard1.neighbors.push(shard3.id);
    shard2.neighbors.push(shard1.id);
    shard2.neighbors.push(shard3.id);
    shard3.neighbors.push(shard1.id);
    shard3.neighbors.push(shard2.id);

    // Connect the parent shard to the child shards
    shards[parent.id].neighbors.push(shard1.id);
    shards[parent.id].neighbors.push(shard2.id);
    shards[parent.id].neighbors.push(shard3.id);
    shard1.neighbors.push(parent.id);
    shard2.neighbors.push(parent.id);
    shard3.neighbors.push(parent.id);

    // Add the child shards to the vector
    shards.push(shard1.clone());
    shards.push(shard2.clone());
    shards.push(shard3.clone());

    // Recursively create the topology for the child shards
    create_sierpinski_topology(child_level, shard1, shards);
    create_sierpinski_topology(child_level, shard2, shards);
    create_sierpinski_topology(child_level, shard3, shards);
}

fn main() {
    let mut shards = Vec::new();
    let root_shard = Shard::new(0, 3); // Adjust the level as needed
    shards.push(root_shard.clone());

    create_sierpinski_topology(3, root_shard, &mut shards); // Adjust the level as needed

    // Print the results
    for shard in &shards {
        println!("Shard {}: Level {}, Neighbors: {:?}", shard.id, shard.level, shard.neighbors);
    }
}
