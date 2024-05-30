use anyhow::Context;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_storage::StoredNode;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};

use crate::errors::TrieCacheError;
use crate::trie_cache::item::CachedItem;

/// Represents a Trie database.
#[derive(Debug, Clone, Copy)]
pub struct TrieDB<'a> {
    conn: &'a PooledConnection<SqliteConnectionManager>,
}

impl<'a> TrieDB<'a> {
    /// Creates a new instance of `TrieDB`.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to a pooled SQLite connection.
    pub fn new(conn: &'a PooledConnection<SqliteConnectionManager>) -> Self {
        Self { conn }
    }

    /// Persists the leaves in the database.
    ///
    /// # Arguments
    ///
    /// * `leaves` - A vector of `CachedItem` representing the leaves to be persisted.
    /// * `batch_id` - The ID of the batch to which the leaves belong.
    ///
    /// # Errors
    ///
    /// Returns a `TrieCacheError` if there was an error persisting the leaves.
    pub fn persist_leaves(
        &self,
        leaves: &Vec<CachedItem>,
        batch_id: u64,
    ) -> Result<(), TrieCacheError> {
        const INSERT_QUERY: &str =
            "INSERT INTO leaves (key, commitment, value, batch_id) VALUES (?1, ?2, ?3, ?4)";

        for item in leaves {
            self.conn
                .execute(
                    INSERT_QUERY,
                    params![
                        item.key.to_be_bytes().to_vec(),
                        item.commitment.to_be_bytes().to_vec(),
                        item.value,
                        &batch_id
                    ],
                )
                .map_err(TrieCacheError::from)?;
        }

        Ok(())
    }

    /// Persists the nodes in the database.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A vector of tuples representing the nodes to be persisted. Each tuple contains a `StoredNode`, a `Felt` hash, and a trie index.
    ///
    /// # Errors
    ///
    /// Returns a `TrieCacheError` if there was an error persisting the nodes.
    pub fn persist_nodes(&self, nodes: Vec<(StoredNode, Felt, u64)>) -> Result<(), TrieCacheError> {
        const INSERT_QUERY: &str =
            "INSERT INTO trie_nodes (hash, data, trie_idx) VALUES (?1, ?2, ?3)";
        let mut write_buffer = [0u8; 256];
        for (node, hash, trie_idx) in nodes {
            let length = node
                .encode(&mut write_buffer)
                .map_err(|_| TrieCacheError::NodeEncodingError)?;
            self.conn
                .execute(
                    INSERT_QUERY,
                    params![
                        hash.to_be_bytes().to_vec(),
                        write_buffer[..length].to_vec(),
                        trie_idx,
                    ],
                )
                .map_err(TrieCacheError::from)?;
        }

        Ok(())
    }

    /// Retrieves the maximum trie index from the database.
    ///
    /// # Errors
    ///
    /// Returns a `TrieCacheError` if there was an error retrieving the trie index.
    pub fn get_node_idx(&self) -> Result<u64, TrieCacheError> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT MAX(trie_idx) FROM trie_nodes")?;

        let trie_idx: Option<u64> = stmt
            .query_row([], |row| row.get::<_, Option<u64>>(0))
            .optional()? // Using optional to handle no rows found situation gracefully
            .flatten(); // Flatten to convert Option<Option<u64>> to Option<u64>

        Ok(trie_idx.map_or(0, |idx| idx))
    }
}

impl Storage for TrieDB<'_> {
    /// Retrieves the stored node at the specified index from the trie database.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to retrieve.
    ///
    /// # Returns
    ///
    /// Returns `Ok(None)` if no node is found at the specified index.
    /// Otherwise, returns `Ok(Some(node))` where `node` is the retrieved stored node.
    fn get(&self, index: u64) -> anyhow::Result<Option<StoredNode>> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT data FROM trie_nodes WHERE trie_idx = ?")
            .context("Creating get statement")?;

        let Some(data): Option<Vec<u8>> = stmt
            .query_row(params![&index], |row| row.get(0))
            .optional()?
        else {
            return Ok(None);
        };

        let node = StoredNode::decode(&data).context("Decoding node")?;

        Ok(Some(node))
    }

    /// Retrieves the hash value of the stored node at the specified index from the trie database.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to retrieve the hash value for.
    ///
    /// # Returns
    ///
    /// Returns `Ok(None)` if no node is found at the specified index.
    /// Otherwise, returns `Ok(Some(hash))` where `hash` is the retrieved hash value.
    fn hash(&self, index: u64) -> anyhow::Result<Option<Felt>> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT hash FROM trie_nodes WHERE trie_idx = ?")?;

        let Some(data): Option<Vec<u8>> = stmt
            .query_row(params![&index], |row| row.get(0))
            .optional()?
        else {
            return Ok(None);
        };

        Ok(Some(Felt::from_be_slice(&data)?))
    }

    /// Retrieves the leaf value associated with the specified path from the trie database.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the leaf to retrieve.
    ///
    /// # Returns
    ///
    /// Returns `Ok(None)` if no leaf is found at the specified path.
    /// Otherwise, returns `Ok(Some(leaf))` where `leaf` is the retrieved leaf value.
    fn leaf(&self, path: &BitSlice<u8, Msb0>) -> anyhow::Result<Option<Felt>> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT commitment FROM leaves WHERE key = ?")
            .context("Creating get statement")?;

        let Some(data): Option<Vec<u8>> = stmt
            .query_row(
                params![Felt::from_bits(path)?.to_be_bytes().to_vec()],
                |row| row.get(0),
            )
            .optional()?
        else {
            return Ok(None);
        };

        Ok(Some(Felt::from_be_slice(&data)?))
    }
}
