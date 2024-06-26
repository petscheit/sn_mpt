use super::item::CachedItem;
use crate::db::trie::TrieDB;
use crate::errors::TrieCacheError;
use crate::trie_cache::batch_proof::{BatchProof, LeafUpdate};
use pathfinder_common::hash::PoseidonHash;
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_merkle_tree::tree::MerkleTree;
use pathfinder_storage::{Node, NodeRef, StoredNode, TrieUpdate};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct Trie {}

/// The Trie struct represents a Merkle Trie data structure.
impl Trie {
    /// Loads a Trie from the given root index and database connection.
    ///
    /// # Arguments
    ///
    /// * `root_idx` - The root index of the Trie.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A tuple containing the TrieDB and the MerkleTree.
    pub fn load(
        root_idx: u64,
        conn: &PooledConnection<SqliteConnectionManager>,
    ) -> (TrieDB, MerkleTree<PoseidonHash, 251>) {
        let storage = TrieDB::new(conn);
        let trie = MerkleTree::<PoseidonHash, 251>::new(root_idx);

        (storage, trie)
    }

    /// Creates a new Trie using the given database connection.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A tuple containing the TrieDB and the MerkleTree.
    pub fn new(
        conn: &PooledConnection<SqliteConnectionManager>,
    ) -> (TrieDB, MerkleTree<PoseidonHash, 251>) {
        let mut trie = MerkleTree::<PoseidonHash, 251>::empty();
        let storage = TrieDB::new(conn);
        // We need to insert and persist a dummy item to initialize the storage for now.
        // ToDo: figure out how to get around this
        let item = CachedItem::new(vec![0; 32]);
        let _ = trie.set(&storage, item.key.view_bits().to_bitvec(), item.commitment);
        let update = trie.clone().commit(&storage).unwrap();
        let _ = Trie::persist_batch_items(storage, &update, &vec![item], &0);

        (storage, trie)
    }

    /// Adds a batch of items to the Database, and generates the required proofs verifying the transition.
    ///
    /// # Arguments
    ///
    /// * `storage` - The TrieDB.
    /// * `trie` - The MerkleTree.
    /// * `root_idx` - The root index of the Trie.
    /// * `items` - The items to be persisted.
    /// * `batch_id` - The batch ID.
    ///
    /// # Returns
    ///
    /// A Result containing the BatchProof and the next index.
    pub fn persist_batch_and_generate_proofs(
        storage: TrieDB,
        mut trie: MerkleTree<PoseidonHash, 251>,
        root_idx: u64,
        items: Vec<CachedItem>,
        batch_id: &u64,
    ) -> Result<(BatchProof, u64), TrieCacheError> {
        let mut leaf_updates: Vec<LeafUpdate> = vec![];
        let mut proofs: Vec<Vec<TrieNode>> = vec![];

        let pre_root = match storage.hash(root_idx) {
            Ok(Some(root)) => root,
            Ok(None) => return Err(TrieCacheError::NodeNotFound),
            Err(e) => return Err(TrieCacheError::from(e)),
        };

        // Write new leafs to tree and generate pre-insert proofs
        items.iter().try_for_each(|item| {
            let proof = MerkleTree::<PoseidonHash, 251>::get_proof(
                root_idx,
                &storage,
                &item.key.view_bits().to_bitvec(),
            )
            .map_err(|_| TrieCacheError::ProofGenerationError)?
            .ok_or(TrieCacheError::ProofGenerationError)?;

            trie.set(&storage, item.key.view_bits().to_bitvec(), item.commitment)
                .map_err(TrieCacheError::from)?;

            leaf_updates.push(item.into());
            proofs.push(proof);

            Ok::<(), TrieCacheError>(())
        })?;

        // Commit update and persist new leafs to storage
        let update = trie.commit(&storage)?;
        Trie::persist_batch_items(storage, &update, &items, batch_id)?;
        let next_index = root_idx + update.nodes_added.len() as u64;

        // Generate post-insert proofs
        items.iter().try_for_each(|item| {
            let proof = MerkleTree::<PoseidonHash, 251>::get_proof(
                next_index,
                &storage,
                &item.key.view_bits().to_bitvec(),
            )
            .map_err(|_| TrieCacheError::ProofGenerationError)?
            .ok_or(TrieCacheError::ProofGenerationError)?;
            proofs.push(proof);
            Ok::<(), TrieCacheError>(())
        })?;

        Ok((
            BatchProof::new::<PoseidonHash>(
                pre_root,
                update.root_commitment,
                leaf_updates,
                proofs,
                batch_id,
            ),
            next_index,
        ))
    }

    /// Persists batch items and corresponding nodes to the TrieDB.
    ///
    /// # Arguments
    ///
    /// * `storage` - The TrieDB.
    /// * `update` - The TrieUpdate.
    /// * `items` - The items to be persisted.
    /// * `batch_id` - The batch ID.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure.
    fn persist_batch_items(
        storage: TrieDB,
        update: &TrieUpdate,
        items: &Vec<CachedItem>,
        batch_id: &u64,
    ) -> Result<(), TrieCacheError> {
        let next_index = storage.get_node_idx()? + 1;
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

        storage.persist_nodes(nodes_to_persist)?;
        storage.persist_leaves(items, *batch_id)?;

        Ok(())
    }
}
