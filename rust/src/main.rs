mod utils;
mod proof;
mod persistance;


use pathfinder_merkle_tree::{tree::MerkleTree};
use pathfinder_common::hash::{FeltHash, PedersenHash};
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use rand::{thread_rng, seq::SliceRandom, Rng};
use crate::utils::{CairoCompatible};
use crate::persistance::Persistance;
use pathfinder_common::felt;
use pathfinder_storage::{Node, NodeRef, StoredNode};


#[derive(Debug)]
pub struct BatchUpdate {
    pub pre_root: Felt,
    pub post_root: Felt,
    pub leaf_updates: Vec<LeafUpdate>,
}
#[derive(Debug)]

pub struct LeafUpdate {
    key: Felt,
    proof_pre: Vec<TrieNode>,
    proof_post: Vec<TrieNode>,
}

#[derive(Debug)]
pub struct MemorizerCache {
    pub root: Felt, // this should be pulled from underlying merkle tree crate
    pub storage: Persistance,
    pub tree: MerkleTree<PedersenHash, 251>,
}

impl Default for MemorizerCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorizerCache {
    pub fn new() -> Self {
        let tree = MerkleTree::<PedersenHash, 251>::empty();
        Self {  tree, storage: Persistance::default(), root: felt!("0x0") }
    }

    pub fn seed(&mut self, seed_size: u64) {
        let mut i: u64 = 0;
        while i < seed_size {
            let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(i));
            let value = PedersenHash::hash(key, key);
            self.tree.set(&self.storage, key.view_bits().to_bitvec(), value).unwrap();
            i += 1;
        }

        let (root, _) = self.commit_and_persist(true);
        self.root = root;
    }

    pub fn create_random_update_batch(&mut self, leaf_count: u64) -> anyhow::Result<BatchUpdate> {
        let root_pre = self.root;
        let mut root_post = self.root;
        let mut leaf_updates = Vec::new();

        let mut rng = thread_rng();

        // Clone keys here to avoid holding references
        let sampled_keys = {
            let keys: Vec<&Felt> = self.storage.leaves.keys().collect();
            let mut shuffled_keys = keys;
            shuffled_keys.shuffle(&mut rng);
            shuffled_keys.into_iter().take(leaf_count as usize).cloned().collect::<Vec<Felt>>()
        };

        for key in sampled_keys {
            let value = PedersenHash::hash(key, Felt::from_u64(rng.gen::<u64>()));
            let (update, root) = self.update_leaf(key, value)?;
            root_post = root;
            leaf_updates.push(update);
        }

        Ok(BatchUpdate {
            pre_root: root_pre,
            post_root: root_post,
            leaf_updates,
        })
    }

    pub fn update_leaf(&mut self, key: Felt, value: Felt) -> anyhow::Result<(LeafUpdate, Felt)> {
        let index = self.storage.next_index - 1;
        let proof_pre = MerkleTree::<PedersenHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();
        self.tree.set(&self.storage, key.view_bits().to_bitvec(), value)?;
        let (root, index) = self.commit_and_persist(true);
        let proof_post = MerkleTree::<PedersenHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();

        let leaf_update = LeafUpdate {
            key,
            proof_pre,
            proof_post,
        };

        Ok((leaf_update, root))
    }

    fn commit_and_persist(
        &mut self,
        prune_nodes: bool,
    ) -> (Felt, u64) {
        for (key, value) in &self.tree.leaves {
            let key = Felt::from_bits(key).unwrap();
            let _ = &self.storage.leaves.insert(key, *value);
        }

        let update = self.tree.clone().commit(&self.storage).unwrap();

        if prune_nodes {
            for idx in update.nodes_removed {
                let _ = &self.storage.nodes.remove(&idx);
            }
        }

        let number_of_nodes_added = update.nodes_added.len() as u64;

        for (rel_index, (hash, node)) in update.nodes_added.into_iter().enumerate() {
            let node = match node {
                Node::Binary { left, right } => {
                    let left = match left {
                        NodeRef::StorageIndex(idx) => idx,
                        NodeRef::Index(idx) => &self.storage.next_index + (idx as u64),
                    };

                    let right = match right {
                        NodeRef::StorageIndex(idx) => idx,
                        NodeRef::Index(idx) => &self.storage.next_index + (idx as u64),
                    };

                    StoredNode::Binary { left, right }
                }
                Node::Edge { child, path } => {
                    let child = match child {
                        NodeRef::StorageIndex(idx) => idx,
                        NodeRef::Index(idx) => &self.storage.next_index + (idx as u64),
                    };

                    StoredNode::Edge { child, path }
                }
                Node::LeafBinary => StoredNode::LeafBinary,
                Node::LeafEdge { path } => StoredNode::LeafEdge { path },
            };

            let index = &self.storage.next_index + (rel_index as u64);

            let _ = &self.storage.nodes.insert(index, (hash, node));
        }

        let storage_root_index = &self.storage.next_index + number_of_nodes_added - 1;
        self.storage.next_index += number_of_nodes_added;

        (update.root_commitment, storage_root_index)
    }
}

fn main() {
    let mut cache = MemorizerCache::new();
    println!("Seeding Merkle...");
    cache.seed(250);

    println!("Creating random update batch...");
    let batch = &cache.create_random_update_batch(10).unwrap();
    println!("{}", batch.to_cairo_str(false));
}