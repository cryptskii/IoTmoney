use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use rand::random;

#[derive(Clone)]
struct Message {
    content: String,
    id: usize,
}

impl Message {
    fn new(content: &str) -> Self {
        Message {
            content: content.to_string(),
            id: random(),
        }
    }
}

struct Node {
    id: u32,
    neighbors: HashMap<u32, Arc<Mutex<Node>>>,
    received_messages: HashSet<usize>,
    message_queue: VecDeque<(Arc<Message>, Arc<Mutex<Network>>)>,
}

impl Node {
    fn new(id: u32) -> Self {
        Node {
            id,
            neighbors: HashMap::new(),
            received_messages: HashSet::new(),
            message_queue: VecDeque::new(),
        }
    }

    fn process_messages(&mut self) {
        while let Some((message, network)) = self.message_queue.pop_front() {
            self.receive_message(&message, &network);
        }
    }

    fn receive_message(&mut self, message: &Arc<Message>, network: &Arc<Mutex<Network>>) {
        if !self.received_messages.contains(&message.id) {
            println!("Node {} received {}", self.id, message.content);
            self.received_messages.insert(message.id);
            network.lock().unwrap().broadcast(self, Arc::clone(message));
        }
    }

    fn send_message(&self, node: &Arc<Mutex<Node>>, message: &Arc<Message>, network: &Arc<Mutex<Network>>) {
        println!("Node {} sent to {}: {}", self.id, node.lock().unwrap().id, message.content);
        node.lock().unwrap().message_queue.push_back((Arc::clone(message), Arc::clone(network)));
    }
}

#[derive(Clone)]
struct Network {
    nodes: HashMap<u32, Arc<Mutex<Node>>>,
    visited: HashSet<u32>,
}

impl Network {
    fn new() -> Self {
        Network {
            nodes: HashMap::new(),
            visited: HashSet::new(),
        }
    }

    fn add_node(&mut self, node: Arc<Mutex<Node>>) {
        let id = node.lock().unwrap().id;
        self.nodes.insert(id, node);
    }

    fn broadcast(&mut self, source: &Node, message: Arc<Message>) {
        if self.visited.contains(&source.id) {
            return;
        }
        self.visited.insert(source.id);

        for neighbor in source.neighbors.values() {
            source.send_message(neighbor, &message, &Arc::new(Mutex::new(self.clone())));
        }
    }
}

fn main() {
    let network = Arc::new(Mutex::new(Network::new()));

    let node1 = Arc::new(Mutex::new(Node::new(1)));
    let node2 = Arc::new(Mutex::new(Node::new(2)));
    let node3 = Arc::new(Mutex::new(Node::new(3)));

    network.lock().unwrap().add_node(Arc::clone(&node1));
    network.lock().unwrap().add_node(Arc::clone(&node2));
    network.lock().unwrap().add_node(Arc::clone(&node3));

    node1.lock().unwrap().neighbors.insert(2, Arc::clone(&node2));
    node1.lock().unwrap().neighbors.insert(3, Arc::clone(&node3));
    node2.lock().unwrap().neighbors.insert(1, Arc::clone(&node1));
    node3.lock().unwrap().neighbors.insert(1, Arc::clone(&node1));

    let message = Arc::new(Message::new("Hello, Network!"));
    network.lock().unwrap().broadcast(&node1.lock().unwrap(), Arc::clone(&message));

    node1.lock().unwrap().process_messages();
    node2.lock().unwrap().process_messages();
    node3.lock().unwrap().process_messages();
}
