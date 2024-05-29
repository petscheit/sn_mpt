use crate::batch_proof::{BatchProof, LeafUpdate};
use crate::items::CachedItem;
use crate::persistance::{CacheDB, Persistance};
use anyhow::anyhow;
use pathfinder_common::hash::FeltHash;
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::tree::MerkleTree;
use pathfinder_storage::{Node, NodeRef, StoredNode, TrieUpdate};
use std::collections::HashMap;
use pathfinder_merkle_tree::storage::Storage;
use crate::cache::BatchStatus;

pub struct CacheTree<H: FeltHash, const HEIGHT: usize> {
    trie: MerkleTree<H, HEIGHT>,
    storage: Persistance,
}

impl<H: FeltHash, const HEIGHT: usize> CacheTree<H, HEIGHT> {
    pub fn new() -> Self {
        let mut cache = CacheTree::<H, HEIGHT> {
            trie: MerkleTree::<H, HEIGHT>::empty(),
            storage: Persistance::new(),
        };

        // We need to insert and persist a dummy item to initialize the storage for now.
        // ToDo: figure out how to get around this
        let item = CachedItem::new(vec![0; 32]);
        let _ = cache.trie.set(
            &cache.storage,
            item.key.view_bits().to_bitvec(),
            item.commitment,
        );
        let update = cache.trie.clone().commit(&cache.storage).unwrap();
        let _ = cache.persist_update(&update, &vec![item], &0);

        cache
    }

    pub fn commit_batch(
        &mut self,
        items: Vec<CachedItem>,
        batch_id: &u64,
    ) -> anyhow::Result<(BatchProof, u64)> {
        let mut leaf_updates: Vec<LeafUpdate> = vec![];
        let mut proofs: Vec<Vec<TrieNode>> = vec![];

        let root_index_pre = self.storage.get_next_node_idx()? - 1;
        let pre_root = self.storage.hash(root_index_pre)?.unwrap_or_else(|| Felt::ZERO);

        // Write new leafs to trie and generate pre-insert proofs
        items.iter().try_for_each(|item| {
            let proof = MerkleTree::<H, HEIGHT>::get_proof(
                root_index_pre,
                &self.storage,
                &item.key.view_bits().to_bitvec(),
            )?
            .ok_or(anyhow!("Pre-insert proof not found"))?;

            self.trie.set(
                &self.storage,
                item.key.view_bits().to_bitvec(),
                item.commitment,
            )?;

            leaf_updates.push(item.into());
            proofs.push(proof);

            Ok::<(), anyhow::Error>(())
        })?;

        // Commit update and persist new leafs to storage
        let update = self.trie.clone().commit(&self.storage)?; // This clone is a crime
        let _ = self.persist_update(&update, &items, batch_id)?;
        let next_index = root_index_pre + update.nodes_added.len() as u64;

        // Generate post-insert proofs
        items.iter().try_for_each(|item| {
            let proof = MerkleTree::<H, HEIGHT>::get_proof(
                next_index,
                &self.storage,
                &item.key.view_bits().to_bitvec(),
            )?
            .ok_or(anyhow!("No proof found"))?;
            proofs.push(proof);
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(
            (BatchProof::new::<H>(
                pre_root,
                update.root_commitment,
                leaf_updates,
                proofs,
                batch_id,
            ),
            next_index)
        )
    }

    fn persist_update(&mut self, update: &TrieUpdate, items: &Vec<CachedItem>, batch_id: &u64) -> anyhow::Result<()> {
        let next_index = self.storage.get_next_node_idx()?;
        let mut nodes_to_persist: Vec<(StoredNode, Felt, u64)> = vec![];

        // Insert new nodes into storage
        for (rel_index, (hash, node)) in update.nodes_added.iter().enumerate() {
            let node = match node {
                Node::Binary { left, right } => {
                    let left = match left {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) => next_index + (*idx as u64),
                    };

                    let right = match right {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) => next_index + (*idx as u64),
                    };

                    StoredNode::Binary { left, right }
                }
                Node::Edge { child, path } => {
                    let child = match child {
                        NodeRef::StorageIndex(idx) => *idx,
                        NodeRef::Index(idx) => next_index + (*idx as u64),
                    };

                    StoredNode::Edge {
                        child,
                        path: path.clone(),
                    }
                }
                Node::LeafBinary => StoredNode::LeafBinary,
                Node::LeafEdge { path } => StoredNode::LeafEdge { path: path.clone() },
            };

            let index = next_index + (rel_index as u64);
            nodes_to_persist.push((node, *hash, index));
        }

        self.storage.persist_nodes(nodes_to_persist)?;
        self.storage.persist_leaves(items, *batch_id)?;

        // Remove leaves from trie memory
        for item in items {
            self.trie.leaves.remove(&item.key.view_bits().to_bitvec());
        }

        Ok(())
    }

    pub fn finalize_batch(&mut self, batch_id: &u64) -> anyhow::Result<()> {
        // Update the status of the batch to reflect that it has been finalized
        self.storage.update_batch_status(batch_id, BatchStatus::Finalized)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::CachedItem;
    use pathfinder_common::hash::PoseidonHash;

    #[test]
    fn commit_batch_successfully_commits_and_updates_state() {
        let mut cache_tree = CacheTree::<PoseidonHash, 10>::new();
        let items: Vec<_> = (0..10).map(|_| CachedItem::default()).collect();

        let result = cache_tree.commit_batch(items, &1).unwrap();

        assert_eq!(
            hex::encode(cache_tree.latest_root.to_be_bytes()),
            result.post_root
        );
        assert_eq!(
            cache_tree.root_to_index.get(&cache_tree.latest_root),
            Some(&4)
        );

        let next_items: Vec<_> = (0..10).map(|_| CachedItem::default()).collect();
        let next_result = cache_tree.commit_batch(next_items, &1).unwrap();

        // ensure the transistion roots are the same
        assert_eq!(result.post_root, next_result.pre_root);
    }
}
