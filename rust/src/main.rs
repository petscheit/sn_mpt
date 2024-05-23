mod utils;
mod proof;
mod persistance;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use pathfinder_merkle_tree::{tree::MerkleTree};
use pathfinder_common::hash::{FeltHash, PoseidonHash};
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use rand::{thread_rng, seq::SliceRandom, Rng};
use crate::utils::{CairoCompatible};
use crate::persistance::Persistance;
use pathfinder_common::felt;
use pathfinder_storage::{Node, NodeRef, StoredNode};
use std::fs::write;
use std::path::Path;

#[derive(Debug)]
pub struct LeafUpdate {
    key: Felt,
    value_pre: Felt,
    value_post: Felt,
    proof_pre: Vec<TrieNode>,
    proof_post: Vec<TrieNode>,
}


#[derive(Debug)]
pub struct BatchUpdate {
    pub pre_root: Felt,
    pub post_root: Felt,
    pub leaf_updates: Vec<LeafUpdate>,
}

#[derive(Serialize)]
pub struct LeafUpdateCairo0 {
    pub key: String,
    pub pre_value: String,
    pub post_value: String,
}

#[derive(Serialize)]
pub struct BatchUpdateCairo0 {
    pub pre_root: String,
    pub post_root: String,
    pub preimage: HashMap<String, Vec<String>>,
    pub leaf_updates: Vec<LeafUpdateCairo0>,
}


#[derive(Debug)]
pub struct MemorizerCache {
    pub root: Felt, // this should be pulled from underlying merkle tree crate
    pub storage: Persistance,
    pub tree: MerkleTree<PoseidonHash, 251>,
}

impl Default for MemorizerCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorizerCache {
    pub fn new() -> Self {
        let tree = MerkleTree::<PoseidonHash, 251>::empty();
        Self {  tree, storage: Persistance::default(), root: felt!("0x0") }
    }

    pub fn seed(&mut self, seed_size: u64) {
        let mut i: u64 = 0;
        while i < seed_size {
            let key = PoseidonHash::hash(felt!("0x0"), Felt::from_u64(i));
            let value = PoseidonHash::hash(key, key);
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
            let value = PoseidonHash::hash(key, Felt::from_u64(rng.gen::<u64>()));
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
        let proof_pre = MerkleTree::<PoseidonHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();
        let value_pre = self.storage.leaves.get(&key).cloned().unwrap_or_default();
        self.tree.set(&self.storage, key.view_bits().to_bitvec(), value)?;
        let (root, index) = self.commit_and_persist(true);
        let proof_post = MerkleTree::<PoseidonHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();

        let leaf_update = LeafUpdate {
            key,
            value_pre,
            value_post: value,
            proof_pre,
            proof_post,
        };

        Ok((leaf_update, root))
    }

    fn add_leaf(&mut self, key: Felt, value: Felt) -> anyhow::Result<BatchUpdate> {
        let root_pre = self.root;
        let value_pre = self.storage.leaves.get(&key).cloned().unwrap_or_default();
        let index = self.storage.next_index - 1;
        let proof_pre = MerkleTree::<PoseidonHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();
        self.tree.set(&self.storage, key.view_bits().to_bitvec(), value)?;
        let (root_post, index) = self.commit_and_persist(true);

        let leaf_update = LeafUpdate {
            key,
            value_pre,
            value_post: value,
            proof_pre,
            proof_post: MerkleTree::<PoseidonHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap(),
        };

        let batch = BatchUpdate {
            pre_root: root_pre,
            post_root: root_post,
            leaf_updates: vec![leaf_update],
        };

        Ok(batch)
    }

     pub fn create_random_batch_leaf_add(&mut self, leaf_count: u64) -> anyhow::Result<BatchUpdate> {
          let mut rng = thread_rng();

         let random_leaf_adds: Vec<(Felt, Felt)> = (0..leaf_count).map(|_| {
             let value = Felt::from_u64(rng.gen::<u64>());
             let key = PoseidonHash::hash(felt!("0x0"), value.clone());
            (key, value)
         }).collect();

         self.batch_add_leaves(random_leaf_adds)
    }

    fn batch_add_leaves(&mut self, new_leafs: Vec<(Felt, Felt)>) -> anyhow::Result<BatchUpdate> {
        let root_pre = self.root;
        let mut leaf_updates: Vec<LeafUpdate>= vec![];
        let index_pre = self.storage.next_index - 1;

        // collect pre add values
        for (key, value) in &new_leafs {
            let mut leaf_proof_pre = MerkleTree::<PoseidonHash, 251>::get_proof(index_pre, &self.storage, &key.view_bits().to_bitvec())?.unwrap();
            let value_pre = self.storage.leaves.get(&key).cloned().unwrap_or_default();

            let leaf_update = LeafUpdate {
                key: *key,
                value_pre,
                proof_pre: leaf_proof_pre.clone(),
                value_post: Default::default(),
                proof_post: vec![],
            };

            self.tree.set(&self.storage, key.view_bits().to_bitvec(), *value)?;

            leaf_updates.push(leaf_update);
        }

        // commit and generate post add proofs
        let (root_post, index) = self.commit_and_persist(true);
        for (i, (key, value)) in new_leafs.iter().enumerate() {
            let leaf_update = &mut leaf_updates[i];
            leaf_update.value_post = *value;
            leaf_update.proof_post = MerkleTree::<PoseidonHash, 251>::get_proof(index, &self.storage, &key.view_bits().to_bitvec())?.unwrap();
        }

        Ok(BatchUpdate {
            pre_root: root_pre,
            post_root: root_post,
            leaf_updates,
        })
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

fn export_to_json<T: serde::Serialize>(data: &T, file_path: &Path) -> anyhow::Result<()> {
    let json_string = to_string_pretty(data)?;
    write(file_path, json_string)?;
    Ok(())
}

fn main() {
    let mut cache = MemorizerCache::new();

    println!("Seeding Merkle...");
    cache.seed(5000);

    println!("Creating random update batch...");
    // let batch = &cache.create_random_batch_leaf_add(10).unwrap();
    let batch = &cache.create_random_batch_leaf_add(3).unwrap();

    let batch_0: BatchUpdateCairo0 = batch.into();

      // Specify the path where you want to save the JSON file
    let file_path = Path::new("output.json");
    export_to_json(&batch_0, file_path);
}
