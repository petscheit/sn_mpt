use anyhow::{anyhow, Context};
use bitvec::prelude::{BitSlice, Msb0};
use num_traits::FromPrimitive;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_storage::StoredNode;
use rusqlite::{OptionalExtension, params};
use crate::cache::{Batch, BatchStatus};
use crate::db::DB;
use crate::items::CachedItem;

#[derive(Debug)]
pub struct Persistance {
    pub db: DB,
}

impl Persistance {
    pub fn new() -> Self {
        let mut db = DB::new("database.db".to_string()).unwrap();
        db.create_table().unwrap();
        Self {
            db,
        }
    }

    pub fn persist_leaves(&self, leaves: &Vec<CachedItem>, batch_id: u64) -> anyhow::Result<()> {
        const INSERT_QUERY: &str = "INSERT INTO leaves (key, commitment, value, batch_id) VALUES (?1, ?2, ?3, ?4)";

        for item in leaves {
            self.db.connection.execute(
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
            self.db.connection.execute(
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

    pub fn get_next_node_idx(&self) -> anyhow::Result<u64> {
        let mut stmt = self.db.connection.prepare_cached(
            "SELECT MAX(trie_idx) FROM trie_nodes",
        )?;

        let trie_idx: Option<u64> = stmt.query_row([], |row| {
            row.get::<_, Option<u64>>(0)
        }).optional()? // Using optional to handle no rows found situation gracefully
            .flatten(); // Flatten to convert Option<Option<u64>> to Option<u64>

        Ok(trie_idx.map_or(0, |idx| idx + 1))
    }
}


pub trait CacheDB {
    fn get_batch(&self, id: u64) -> anyhow::Result<Batch>;
    fn get_finalized_batch(&self) -> anyhow::Result<Option<Batch>>;
    fn get_latest_batch(&self) -> anyhow::Result<Option<Batch>>;
    fn create_batch(&self, parent_id: Option<u64>, root_idx: u64) -> anyhow::Result<u64>;
    fn update_batch_status(&self, id: &u64, new_status: BatchStatus) -> anyhow::Result<()>;
}

impl CacheDB for Persistance {
    fn get_batch(&self, id: u64) -> anyhow::Result<Batch> {
        let mut stmt = self.db.connection.prepare_cached(
            "SELECT id, parent_id, status, root_idx FROM batches WHERE id = ?",
        )?;
        let (id, parent_id, status, root_idx): (u64, Option<u64>, u64, u64) = stmt.query_row(params![id], |row| {
            Ok((row.get(0)?, row.get(1)?,row.get(2)?, row.get(3)?))
        })?;

        Ok(Batch {
            id,
            parent_id,
            status: BatchStatus::from_u64(status).context("Invalid batch status")?,
            root_idx
        })
    }
    fn get_finalized_batch(&self) -> anyhow::Result<Option<Batch>> {
        let status = BatchStatus::Finalized as u64; // Ensure this cast is valid as per your enum definition
        let mut stmt = self.db.connection.prepare_cached(
            "SELECT id, parent_id, status, root_idx FROM batches WHERE status = ?"
        )?;

        // Execute the query and attempt to fetch the row
        let batch = stmt.query_row(params![status], |row| {
            Ok(Batch {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                status: BatchStatus::from_u64(row.get(2)?).unwrap(),
                root_idx: row.get(3)?,
            })
        }).optional()?; // Use optional to handle the case where no rows are found

        Ok(batch)
    }


    fn get_latest_batch(&self) -> anyhow::Result<Option<Batch>> {
        let status = BatchStatus::Created as u64;
        let mut stmt = self.db.connection.prepare_cached(
            "SELECT id, parent_id, status, root_idx FROM batches WHERE status = ?"
        )?;

        let batch = stmt.query_row(params![status], |row| {
            Ok(Batch {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                status: BatchStatus::from_u64(row.get(2)?).unwrap(),
                root_idx: row.get(3)?,
            })
        }).optional()?;

        Ok(batch)
    }

    fn create_batch(&self, parent_id: Option<u64>, root_idx: u64) -> anyhow::Result<u64> {
        const INSERT_QUERY: &str = "INSERT INTO batches (parent_id, status, root_idx) VALUES (?, ?, ?)";
        self.db.connection.execute(
            INSERT_QUERY,
            params![parent_id, BatchStatus::Created as u64, root_idx],
        )?;
        Ok(self.db.connection.last_insert_rowid() as u64)
    }

    fn update_batch_status(&self, id: &u64, new_status: BatchStatus) -> anyhow::Result<()> {
        let updated_rows = self.db.connection.execute(
            "UPDATE batches SET status = ?1 WHERE id = ?2",
            params![new_status as u64, id],
        )?;

        if updated_rows == 0 {
            Err(anyhow!("No rows were updated"))
        } else {
            Ok(())
        }
    }
}

impl Storage for Persistance {
    fn get(&self, index: u64) -> anyhow::Result<Option<StoredNode>> {
        let mut stmt = self
            .db
            .connection
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
        let mut stmt = self
            .db
            .connection
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
        let mut stmt = self
            .db
            .connection
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