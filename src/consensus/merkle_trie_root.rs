use merkletree::MerkleTree;

pub fn get_merkle_root(leaves: &[Vec<u8>]) -> Vec<u8> {
  let tree = MerkleTree::from_vec(leaves.to_vec());
  tree.root()
}