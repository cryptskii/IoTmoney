use rand::Rng;
use serde::{Deserialize, Serialize};

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
