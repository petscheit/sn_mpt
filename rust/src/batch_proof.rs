use crate::items::CachedItem;
use pathfinder_common::hash::FeltHash;
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use serde::Serialize;

use std::collections::HashMap;
use serde_json::to_string_pretty;

#[derive(Serialize, Debug)]
pub struct LeafUpdate {
    pub key: String,
    pub pre_value: String,
    pub post_value: String,
}

impl From<&CachedItem> for LeafUpdate {
    fn from(item: &CachedItem) -> Self {
        LeafUpdate {
            key: hex::encode(item.key.to_be_bytes()),
            pre_value: hex::encode(Felt::ZERO.to_be_bytes()),
            post_value: hex::encode(item.commitment.to_be_bytes()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BatchProof {
    pub id: u64,
    pub pre_root: String,
    pub post_root: String,
    pub preimage: HashMap<String, Vec<String>>,
    pub leaf_updates: Vec<LeafUpdate>,
}

impl BatchProof {
    pub fn new<H: FeltHash>(
        pre_root: Felt,
        post_root: Felt,
        leaf_updates: Vec<LeafUpdate>,
        proofs: Vec<Vec<TrieNode>>,
        batch_id: &u64,
    ) -> Self {
        let mut batch_proof = BatchProof {
            id: batch_id.clone(),
            pre_root: hex::encode(pre_root.to_be_bytes()),
            post_root: hex::encode(post_root.to_be_bytes()),
            preimage: HashMap::new(),
            leaf_updates,
        };
        batch_proof.generate_preimage_and_updates::<H>(proofs);
        batch_proof
    }

    fn generate_preimage_and_updates<H: FeltHash>(&mut self, proofs: Vec<Vec<TrieNode>>) {
        proofs.iter().flat_map(|r| r.iter()).for_each(|node| {
            let hash = node.hash::<H>();
            match node {
                TrieNode::Binary { left, right } => {
                    let _ = &self.preimage.insert(
                        hex::encode(hash.to_be_bytes()),
                        vec![
                            hex::encode(left.to_be_bytes()),
                            hex::encode(right.to_be_bytes()),
                        ],
                    );
                }
                TrieNode::Edge { child, path } => {
                    let _ = &self.preimage.insert(
                        hex::encode(hash.to_be_bytes()),
                        vec![
                            hex::encode(path.len().to_be_bytes()),
                            hex::encode(Felt::from_bits(path).unwrap().to_be_bytes()),
                            hex::encode(child.to_be_bytes()),
                        ],
                    );
                }
            }
        });
    }

    pub fn to_json(&self) -> String {
        to_string_pretty(&self).unwrap()
    }
}
