use ed25519_dalek::{Keypair, Signer, Verifier};

pub fn generate_keypair() -> Keypair {
  Keypair::generate(&mut rand::thread_rng())
}

pub fn sign(keypair: &Keypair, message: &[u8]) -> Vec<u8> {
  let signature = keypair.sign(message);
  signature.to_bytes().to_vec()
}

pub fn verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
  let public_key = ed25519_dalek::PublicKey::from_bytes(public_key).unwrap();
  let signature = ed25519_dalek::Signature::from_bytes(signature).unwrap();

  public_key.verify(message, &signature).is_ok()
}