use std::collections::HashMap;
use anyhow::anyhow;
use pathfinder_common::hash::FeltHash;
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_merkle_tree::tree::MerkleTree;
use pathfinder_storage::{Node, NodeRef, StoredNode, TrieUpdate};
use crate::batch_proof::{BatchProof, LeafUpdate};
use crate::items::CachedItem;
use crate::persistance::Persistance;

pub struct CacheTree<H: FeltHash, const HEIGHT: usize> {
    trie: MerkleTree<H, HEIGHT>,
    storage: Persistance,
    latest_root: Felt,
    root_to_index: HashMap<Felt, u64>,
}

impl <H: FeltHash, const HEIGHT: usize> CacheTree<H, HEIGHT> {
    pub fn new() -> Self {

        let mut cache = CacheTree::<H, HEIGHT> {
            trie: MerkleTree::<H, HEIGHT>::empty(),
            storage: Persistance::default(),
            latest_root: Felt::ZERO,
            root_to_index: HashMap::new(),
        };

        // We need to insert and persist a dummy item to initialize the storage for now.
        // ToDo: figure out how to get around this
        let item = CachedItem::new(vec![0; 32]);
        let _ = cache.trie.set(&cache.storage, item.key.view_bits().to_bitvec(), item.commitment);
        let update = cache.trie.clone().commit(&cache.storage).unwrap();
        let _ = cache.persist_update(&update, &0);

        cache.latest_root = update.root_commitment;
        cache.root_to_index.insert(cache.latest_root, 0);

        cache

    }

    pub fn commit_batch(&mut self, items: Vec<CachedItem>, _batch_id: &u64) -> anyhow::Result<BatchProof> {
        let mut leaf_updates: Vec<LeafUpdate> = vec![];
        let mut proofs: Vec<Vec<TrieNode>> = vec![];

        let root_index_pre = *self.root_to_index.get(&self.latest_root).unwrap_or(&0);
        let pre_root = self.latest_root.clone();

        // Write new leafs to trie and generate pre-insert proofs
        &items.iter().map(|item| {
            let proof = MerkleTree::<H, HEIGHT>::get_proof(root_index_pre, &self.storage, &item.key.view_bits().to_bitvec())?.ok_or(anyhow!("Pre-insert proof not found"))?;
            self.trie.set(&self.storage, item.key.view_bits().to_bitvec(), item.commitment)?;

            leaf_updates.push(item.into());
            proofs.push(proof);

            Ok(())
        }).collect::<anyhow::Result<()>>()?;

        // Commit update and persist new leafs to storage
        let update = self.trie.clone().commit(&self.storage)?; // This clone is a crime
        let next_index = self.persist_update(&update, _batch_id)?;

        // Generate post-insert proofs
        items.into_iter().map(|item| {
            let proof = MerkleTree::<H, HEIGHT>::get_proof(next_index, &self.storage, &item.key.view_bits().to_bitvec())?.ok_or(anyhow!("No proof found"))?;
            proofs.push(proof);
            Ok(())
        }).collect::<anyhow::Result<()>>()?;

        // Update local state to reflect the new root
        self.latest_root = update.root_commitment;
        self.root_to_index.insert(self.latest_root, next_index);

        Ok(BatchProof::new::<H>(pre_root, self.latest_root, leaf_updates, proofs))
    }

    fn persist_update(&mut self, update: &TrieUpdate, _batch_id: &u64) -> anyhow::Result<u64> {
        // Insert new leaves into storage
        for (key, value) in &self.trie.leaves {
            let key = Felt::from_bits(key).unwrap();
            let _ = &self.storage.leaves.insert(key, *value);
            // ToDo: Probably we should empty the trie leaves here
        }

        //  if prune_nodes {
        //     for idx in update.nodes_removed {
        //         let _ = &self.storage.nodes.remove(&idx);
        //     }
        // }

        // Insert new nodes into storage
        for (rel_index, (hash, node)) in update.nodes_added.iter().enumerate() {
            let node = match node {
                Node::Binary { left, right } => {
                    let left = match left {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) =>  &self.storage.next_index + (*idx as u64),
                    };

                    let right = match right {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) => &self.storage.next_index + (*idx as u64),
                    };

                    StoredNode::Binary { left, right }
                }
                Node::Edge { child, path } => {
                    let child = match child {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) => &self.storage.next_index + (*idx as u64),
                    };

                    StoredNode::Edge { child, path: path.clone() }
                }
                Node::LeafBinary => StoredNode::LeafBinary,
                Node::LeafEdge { path } => StoredNode::LeafEdge { path: path.clone() },
            };

            let index = &self.storage.next_index + (rel_index as u64);

            let _ = &self.storage.nodes.insert(index, (*hash, node));
        }

        // Update local state to reflect the new root
        let number_of_nodes_added = update.nodes_added.len() as u64;
        let storage_root_index = &self.storage.next_index + number_of_nodes_added - 1;
        self.storage.next_index += number_of_nodes_added;

        Ok(storage_root_index)
    }

    pub fn finalize_batch(&mut self, _batch_id: &u64) -> anyhow::Result<()> {
        // Update the status of the batch to reflect that it has been finalized
        Ok(())
    }
}