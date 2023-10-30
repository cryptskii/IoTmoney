use blake3::hash;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Transaction {
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct EncodedShard {
    data: Vec<u8>,
    parity: Vec<u8>,
    tx_refs: HashSet<u64>,
}

impl EncodedShard {
    fn retain_tx_refs(&mut self, other_shards: &[Vec<Transaction>]) {
        for shard in other_shards {
            for tx in shard {
                let tx_hash = hash(&tx.data).as_bytes().to_vec();
                let tx_id = u64::from_ne_bytes(tx_hash.try_into().expect("Failed to convert hash to u64"));
                self.tx_refs.insert(tx_id);
            }
        }
    }
}

fn encode_txs(tx_shard: &[Transaction]) -> Vec<u8> {
    tx_shard.iter().flat_map(|tx| tx.data.clone()).collect()
}

fn generate_parity(tx_shard: &[Transaction]) -> Vec<u8> {
    // Dummy parity generation logic
    tx_shard.iter().map(|_| 0u8).collect()
}

fn verify_data(data: &[u8]) -> bool {
    !data.is_empty()
}

fn verify_parity(data: &[u8], parity: &[u8]) -> bool {
    data.len() == parity.len()
}

fn encode(transactions: Vec<Transaction>, other_shards: &[Vec<Transaction>]) -> Vec<EncodedShard> {
    let k = 2;
    let mut shards = vec![];

    for tx_shard in transactions.chunks(k) {
        let mut shard = EncodedShard {
            data: encode_txs(tx_shard),
            parity: generate_parity(tx_shard),
            tx_refs: HashSet::new(),
        };

        shard.retain_tx_refs(other_shards);
        shards.push(shard);
    }

    shards
}

fn verify(shards: &[EncodedShard], other_shards: &[Vec<Transaction>]) {
    for shard in shards {
        assert!(verify_data(&shard.data));
        assert!(verify_parity(&shard.data, &shard.parity));

        for tx_ref in &shard.tx_refs {
            let mut found = false;
            for other_shard in other_shards {
                for tx in other_shard {
                    let tx_hash = hash(&tx.data).as_bytes().to_vec();
                    let tx_id = u64::from_ne_bytes(tx_hash.try_into().expect("Failed to convert hash to u64"));
                    if *tx_ref == tx_id {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            assert!(found, "Transaction reference not found in other shards");
        }
    }
}

fn main() {
    let transactions = vec![
        Transaction { data: vec![1, 2, 3] },
        Transaction { data: vec![7, 8, 9] },
        Transaction { data: vec![10, 11, 12] },
        Transaction { data: vec![13, 14, 15] },
    ];

    let other_shards = vec![
        vec![
            Transaction { data: vec![4, 5, 6] },
            Transaction { data: vec![16, 17, 18] },
        ],
        vec![
            Transaction { data: vec![19, 20, 21] },
            Transaction { data: vec![22, 23, 24] },
        ],
    ];

    let encoded_shards = encode(transactions, &other_shards);
    verify(&encoded_shards, &other_shards);

    println!("Verification successful!");
}
