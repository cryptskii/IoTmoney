use crate::transaction::Transaction;
use crate::checksum::Checksum;

pub struct Block {
    transactions: Vec<Transaction>,
    checksum: Checksum,
    // Other block details
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let checksum = Checksum::calculate(&transactions);  // Assuming a suitable conversion to bytes
        Self { transactions, checksum }
    }

    pub fn validate(&self) -> bool {
        // Validate the block's transactions and checksum
        self.transactions.iter().all(Transaction::validate) && self.checksum.verify(&self.transactions)
    }

    pub fn process(&self) {
        // Process the block's transactions
        for transaction in &self.transactions {
            transaction.process();
        }
    }
}
