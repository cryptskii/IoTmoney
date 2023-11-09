use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task;

struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
    signature: String,
    // Other relevant transaction details
}

impl Transaction {
    async fn validate(&self, account_balances: &HashMap<String, u64>) -> Result<(), &'static str> {
        // Check if the sender has enough balance for the transaction
        let sender_balance = account_balances.get(&self.sender).unwrap_or(&0);
        if *sender_balance < self.amount {
            return Err("Insufficient balance");
        }

        // Check if the signature is valid (this is a simplified example)
        if self.signature != self.calculate_signature() {
            return Err("Invalid signature");
        }

        // Additional validation checks can be added here

        Ok(())
    }

    fn calculate_signature(&self) -> String {
        // In a real-world scenario, you would use cryptographic functions to calculate the signature
        // Here, we're just returning a dummy value for simplicity
        "valid_signature".to_string()
    }
}

async fn validate_transactions(transactions: Vec<Transaction>, account_balances: HashMap<String, u64>) {
    let (tx, mut rx) = mpsc::channel(32);

    for transaction in transactions {
        let tx = tx.clone();
        let account_balances = account_balances.clone();
        task::spawn(async move {
            match transaction.validate(&account_balances).await {
                Ok(_) => {
                    println!("Transaction from {} to {} for amount {} is valid.", transaction.sender, transaction.receiver, transaction.amount);
                }
                Err(err) => {
                    println!("Transaction from {} to {} for amount {} is invalid: {}", transaction.sender, transaction.receiver, transaction.amount, err);
                }
            }
            tx.send(()).await.unwrap();
        });
    }

    drop(tx);

    while rx.recv().await.is_some() {}
}

#[tokio::main]
async fn main() {
    // Sample account balances
    let mut account_balances = HashMap::new();
    account_balances.insert("Alice".to_string(), 100);
    account_balances.insert("Bob".to_string(), 50);
    account_balances.insert("Charlie".to_string(), 150);

    // Sample transactions
    let transactions = vec![
        Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 30,
            signature: "valid_signature".to_string(),
        },
        Transaction {
            sender: "Bob".to_string(),
            receiver: "Charlie".to_string(),
            amount: 60,
            signature: "invalid_signature".to_string(),
        },
        Transaction {
            sender: "Charlie".to_string(),
            receiver: "Alice".to_string(),
            amount: 20,
            signature: "valid_signature".to_string(),
        },
    ];

    validate_transactions(transactions, account_balances).await;
}
