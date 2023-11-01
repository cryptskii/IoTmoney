use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    data: String,
    valid: bool,
}

struct FraudDetection {
    proofs: Vec<Proof>,
}

impl FraudDetection {
    fn new() -> Self {
        Self { proofs: Vec::new() }
    }

    fn add_proof(&mut self, proof: Proof) {
        self.proofs.push(proof);
    }

    fn sample_and_check_proofs(&self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let sample_size = (self.proofs.len() as f64 * 0.1).ceil() as usize; // 10% sample size

        let mut invalid_count = 0;
        for _ in 0..sample_size {
            let index = rng.gen_range(0..self.proofs.len());
            if !self.proofs[index].valid {
                invalid_count += 1;
            }
        }

        if invalid_count >= 5 { // Threshold for banning
            Err("Fraud detected: Too many invalid proofs")
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut fraud_detection = FraudDetection::new();

    // Add some dummy proofs (replace with actual proofs in real-world scenario)
    fraud_detection.add_proof(Proof { data: "Proof 1".to_string(), valid: true });
    fraud_detection.add_proof(Proof { data: "Proof 2".to_string(), valid: false });
    fraud_detection.add_proof(Proof { data: "Proof 3".to_string(), valid: true });
    fraud_detection.add_proof(Proof { data: "Proof 4".to_string(), valid: false });
    fraud_detection.add_proof(Proof { data: "Proof 5".to_string(), valid: true });

    // Sample and check proofs
    match fraud_detection.sample_and_check_proofs() {
        Ok(_) => println!("No fraud detected"),
        Err(err) => println!("{}", err),
    }
}


use crate::shard::Shard;
use rand::prelude::*;

pub struct FraudProof;

pub fn propagate_fraud_proof(shard: &Shard, fraud_proof: &FraudProof) {
    // Propagate the fraud proof to the current shard
    // ...

    // Recursively propagate the fraud proof to child shards (if any)
    for &neighbor_id in &shard.neighbors {
        let neighbor_shard = get_shard(neighbor_id);  // Assume a function to retrieve a shard by ID
        propagate_fraud_proof(&neighbor_shard, fraud_proof);
    }
}

pub fn sample_fraud_proofs(shard: &Shard) -> Option<FraudProof> {
    let mut rng = thread_rng();
    if rng.gen::<f64>() < SAMPLE_RATE {  // SAMPLE_RATE is a constant defining the sampling probability
        // Sample a fraud proof from the current shard
        // ...

        // Recursively sample fraud proofs from child shards (if any)
        for &neighbor_id in &shard.neighbors {
            let neighbor_shard = get_shard(neighbor_id);  // Assume a function to retrieve a shard by ID
            if let Some(fraud_proof) = sample_fraud_proofs(&neighbor_shard) {
                return Some(fraud_proof);
            }
        }
    }

    None
}


pub struct FraudDetector {
  sample_rate: f32,
}

// Implementation

pub fn check_fraud(detector: &FraudDetector, proofs: &[Proof]) -> bool {
  detector.sample_and_check(proofs)  
}

struct Proof; // Proof data structure