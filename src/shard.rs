use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::future::join_all;

#[derive(Debug, Clone)]
struct Transaction {
    nonce: u64,
    sender: String,
    recipient: String,
    value: u64,
    // ... other fields
}

#[derive(Debug, Default)]
struct ShardState {
    balances: HashMap<String, u64>,
    // ... other fields
}

async fn syntax_validation(tx: Transaction) -> Result<Transaction, &'static str> {
    if tx.nonce == 0 || tx.value == 0 {
        return Err("Invalid transaction syntax");
    }
    Ok(tx)
}

async fn semantic_validation(tx: Transaction, state: Arc<RwLock<ShardState>>) -> Result<Transaction, &'static str> {
    let state = state.read().await;
    let balance = state.balances.get(&tx.sender).unwrap_or(&0);
    if *balance < tx.value {
        return Err("Insufficient balance");
    }
    Ok(tx)
}

async fn state_transition(tx: Transaction, state: Arc<RwLock<ShardState>>) -> Result<(), &'static str> {
    let mut state = state.write().await;
    let sender_balance = state.balances.entry(tx.sender.clone()).or_insert(0);
    if *sender_balance < tx.value {
        return Err("Insufficient balance");
    }
    *sender_balance -= tx.value;
    let recipient_balance = state.balances.entry(tx.recipient.clone()).or_insert(0);
    *recipient_balance += tx.value;
    Ok(())
}

async fn process_batch(txs: Vec<Transaction>, state: Arc<RwLock<ShardState>>) {
    let mut handles = Vec::new();
    for tx in txs {
        let state_clone = Arc::clone(&state);
        handles.push(tokio::spawn(async move {
            match syntax_validation(tx).await {
                Ok(tx) => match semantic_validation(tx, state_clone.clone()).await {
                    Ok(tx) => {
                        if let Err(e) = state_transition(tx, state_clone).await {
                            println!("State transition error: {}", e);
                        }
                    }
                    Err(e) => println!("Semantic validation error: {}", e),
                },
                Err(e) => println!("Syntax validation error: {}", e),
            }
        }));
    }
    join_all(handles).await;
}

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(ShardState::default()));
    state.write().await.balances.insert("Alice".to_string(), 100);
    state.write().await.balances.insert("Bob".to_string(), 50);

    let transactions = vec![
        Transaction {
            nonce: 1,
            sender: "Alice".to_string(),
            recipient: "Bob".to_string(),
            value: 10,
        },
        Transaction {
            nonce: 2,
            sender: "Alice".to_string(),
            recipient: "Bob".to_string(),
            value: 20,
        },
        Transaction {
            nonce: 3,
            sender: "Bob".to_string(),
            recipient: "Alice".to_string(),
            value: 5,
        },
    ];

    process_batch(transactions, state.clone()).await;

    println!("Final state: {:?}", state.read().await);
}
