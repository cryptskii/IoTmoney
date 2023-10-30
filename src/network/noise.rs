// Import crates
use noise::{NoiseBuilder, NoisePattern, CipherSuite, HashAlgorithm};
use noise_static_keys::X25519; 

// Generate static X25519 key pair    
let (public_key, private_key) = X25519::new();

// Initialize Noise configuration
let noise = NoiseBuilder::new()
  .pattern(NoisePattern::XX)
  .cipher(CipherSuite::ChaChaPoly) 
  .hash(HashAlgorithm::SHA256)
  .build()?;
  
// Perform handshake between Alice and Bob  
let alice = noise.handshake(bob_public_key, alice_private_key)?;
let bob = noise.handshake(alice_public_key, bob_private_key)?;

// Encrypt payload
let payload = b"Hello world!";
let ciphertext = alice.encrypt_message(payload.as_ref());

// Send encrypted message
bob.send_message(ciphertext.as_ref());

// Receive and decrypt message 
let received = bob.recv_message()?;
let decrypted = alice.decrypt_message(received.as_ref())?;

assert_eq!(payload, decrypted);