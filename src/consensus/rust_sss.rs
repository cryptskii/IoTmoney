use threshold_secret_sharing::{Share, SecretSharing};

pub fn split_secret(secret: &[u8], threshold: u8) -> Vec<Share> {
  let secret_sharing = SecretSharing::new(threshold, secret.len(), &mut rand::thread_rng());
  secret_sharing.share(secret).unwrap()  
}

pub fn reconstruct_secret(shares: &[Share]) -> Option<Vec<u8>> {
  if shares.len() < 2 {
     return None;
  }
  
  let secret_sharing = SecretSharing::new(shares.len() as u8, shares[0].len(), &mut rand::thread_rng());
  secret_sharing.reconstruct(shares)  
}