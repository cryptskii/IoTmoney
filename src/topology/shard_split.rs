use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Shard {
  id: u32,
  nodes: Vec<Node>,
  neighbors: HashMap<u32, Shard>, 
}

#[derive(Clone, Debug)]
struct Node {
  id: u32,
  shard: u32,
}

impl Shard {
  fn new(id: u32) -> Self {
    Shard {
      id,
      nodes: Vec::new(),
      neighbors: HashMap::new(),
    }
  }

  fn subdivide(&self) -> (Shard, Shard, Shard) {
    let new_id_1 = self.id + 1;
    let new_id_2 = self.id + 2;
    let new_id_3 = self.id + 3;

    let shard_1 = Shard::new(new_id_1);
    let shard_2 = Shard::new(new_id_2);
    let shard_3 = Shard::new(new_id_3);

    (shard_1, shard_2, shard_3)
  }

  fn add_node(&mut self) {
    let new_node_id = self.nodes.len() as u32 + 1;
    let new_node = Node { id: new_node_id, shard: self.id };
    self.nodes.push(new_node);
  }

  fn add_neighbor(&mut self, neighbor: Shard) {
    self.neighbors.insert(neighbor.id, neighbor);
  }
}

fn main() {
  let mut root_shard = Shard::new(0);

  // Add some nodes to the root shard
  root_shard.add_node();
  root_shard.add_node();

  // Subdivide the root shard
  let (shard_1, shard_2, shard_3) = root_shard.subdivide();

  // Add the new shards as neighbors of the root shard
  root_shard.add_neighbor(shard_1.clone());
  root_shard.add_neighbor(shard_2.clone());
  root_shard.add_neighbor(shard_3.clone());

  // Print out the IDs of the root shard's neighbors
  for (neighbor_id, _neighbor) in &root_shard.neighbors {
    println!("Root shard has neighbor with ID: {}", neighbor_id);
}

  // Print out the nodes in the root shard
  for node in &root_shard.nodes {
    println!("Node ID: {}, Shard ID: {}", node.id, node.shard);
  }
}