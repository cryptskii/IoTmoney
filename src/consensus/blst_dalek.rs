use blst::{PublicKey, Signature};

pub fn aggregate_signatures(signatures: &[Signature]) -> Signature {
  Signature::aggregate(signatures).unwrap()
}

pub fn verify_signature(public_keys: &[PublicKey], message: &[u8], signature: &Signature) -> bool {
  signature.verify_aggregate(public_keys, message).is_ok()  
}