use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::RwLock;
use std::collections::{HashSet, VecDeque};
use log::info;
use async_recursion::async_recursion;

#[derive(Debug, Clone)]
struct Message {
    content: String,
    id: usize,
}

impl Message {
    fn new(content: &str, id: usize) -> Self {
        Message {
            content: content.to_string(),
            id,
        }
    }
}

#[derive(Debug)]
struct Node {
    id: usize,
    message_queue: RwLock<VecDeque<Message>>,
    received_messages: RwLock<HashSet<usize>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Node {
            id,
            message_queue: RwLock::new(VecDeque::new()),
            received_messages: RwLock::new(HashSet::new()),
        }
    }

    async fn send_message(&self, message: Message) {
        self.message_queue.write().await.push_back(message);
    }

    async fn process_messages(&self) {
        let mut messages = self.message_queue.write().await;
        let mut received_messages = self.received_messages.write().await;
        while let Some(message) = messages.pop_front() {
            if !received_messages.contains(&message.id) {
                info!("Node {} received: {}", self.id, message.content);
                received_messages.insert(message.id);
            }
        }
    }

    async fn clone_node(&self) -> Node {
        let message_queue = self.message_queue.read().await.clone();
        let received_messages = self.received_messages.read().await.clone();

        Node {
            id: self.id,
            message_queue: RwLock::new(message_queue),
            received_messages: RwLock::new(received_messages),
        }
    }
}

#[derive(Debug, Clone)]
struct Shard {
    nodes: DashMap<usize, Arc<Node>>,
    children: Vec<Arc<Shard>>,
}

impl Shard {
    fn new() -> Self {
        Shard {
            nodes: DashMap::new(),
            children: Vec::new(),
        }
    }
}

#[async_recursion]
async fn broadcast_message(shard: Arc<Shard>, message: Message) {
    for node in shard.nodes.iter() {
        node.value().send_message(message.clone()).await;
    }
    for child in &shard.children {
        broadcast_message(child.clone(), message.clone()).await;
    }
}

#[async_recursion]
async fn process_messages(shard: Arc<Shard>) {
    for node in shard.nodes.iter() {
        let node_clone = node.value().clone_node().await;
        tokio::spawn(async move {
            node_clone.process_messages().await;
        });
    }
    for child in &shard.children {
        process_messages(child.clone()).await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let root_shard = Arc::new(Shard::new());
    let node1 = Arc::new(Node::new(1));
    let node2 = Arc::new(Node::new(2));

    root_shard.nodes.insert(node1.id, node1.clone());
    root_shard.nodes.insert(node2.id, node2.clone());

    let message = Message::new("Hello, World!", 1);
    broadcast_message(root_shard.clone(), message).await;
    process_messages(root_shard.clone()).await;
}
