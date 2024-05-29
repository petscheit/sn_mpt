use std::sync::Arc;
use anyhow::Context;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_storage::StoredNode;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, OptionalExtension, params};
use crate::db::ConnectionManager;
use crate::trie_cache::item::CachedItem;

#[derive(Debug, Clone, Copy)]
pub struct TrieDB<'a> {
    conn: &'a PooledConnection<SqliteConnectionManager>,
}

impl<'a> TrieDB<'a> {
    pub fn new(conn: &'a PooledConnection<SqliteConnectionManager>) -> Self {
        Self { conn }
    }

    pub fn persist_leaves(&self, leaves: &Vec<CachedItem>, batch_id: u64) -> anyhow::Result<()> {
        const INSERT_QUERY: &str = "INSERT INTO leaves (key, commitment, value, batch_id) VALUES (?1, ?2, ?3, ?4)";

        for item in leaves {
            self.conn.execute(
                INSERT_QUERY,
                params![
                    item.key.to_be_bytes().to_vec(),
                    item.commitment.to_be_bytes().to_vec(),
                    item.value,
                    &batch_id
                ]
            )?;
        }

        Ok(())
    }

    pub fn persist_nodes(&self, nodes: Vec<(StoredNode, Felt, u64)>) -> anyhow::Result<()> {
        const INSERT_QUERY: &str = "INSERT INTO trie_nodes (hash, data, trie_idx) VALUES (?1, ?2, ?3)";
        let mut write_buffer = [0u8; 256];
        for (node, hash, trie_idx) in nodes {
            let length = node.encode(&mut write_buffer).context("Encoding node")?;
            self.conn.execute(
                INSERT_QUERY,
                params![
                    hash.to_be_bytes().to_vec(),
                    write_buffer[..length].to_vec(),
                    trie_idx,
                ],
            )?;
        }

        Ok(())
    }

    pub fn get_node_idx(&self) -> anyhow::Result<u64> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT MAX(trie_idx) FROM trie_nodes",
        )?;

        let trie_idx: Option<u64> = stmt.query_row([], |row| {
            row.get::<_, Option<u64>>(0)
        }).optional()? // Using optional to handle no rows found situation gracefully
            .flatten(); // Flatten to convert Option<Option<u64>> to Option<u64>

        Ok(trie_idx.map_or(0, |idx| idx))
    }
}

impl Storage for TrieDB<'_> {
    fn get(&self, index: u64) -> anyhow::Result<Option<StoredNode>> {
        let mut stmt = self.conn
            .prepare_cached(&format!("SELECT data FROM trie_nodes WHERE trie_idx = ?"))
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

    fn hash(&self, index: u64) -> anyhow::Result<Option<Felt>> {
        let mut stmt =  self.conn
            .prepare_cached(&format!("SELECT hash FROM trie_nodes WHERE trie_idx = ?"))
            .context("Creating get statement")?;

        let Some(data): Option<Vec<u8>> = stmt
            .query_row(params![&index], |row| row.get(0))
            .optional()?
            else {
                return Ok(None);
            };

        Ok(Some(Felt::from_be_slice(&data)?))
    }

    fn leaf(&self, path: &BitSlice<u8, Msb0>) -> anyhow::Result<Option<Felt>> {
        let mut stmt = self.conn
            .prepare_cached(&format!("SELECT commitment FROM leaves WHERE key = ?"))
            .context("Creating get statement")?;

        let Some(data): Option<Vec<u8>> = stmt
            .query_row(params![Felt::from_bits(path)?.to_be_bytes().to_vec()], |row| row.get(0))
            .optional()?
            else {
                return Ok(None);
            };

        Ok(Some(Felt::from_be_slice(&data)?))
    }
}