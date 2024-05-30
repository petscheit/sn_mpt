pub mod batch_proof;
pub mod item;
pub mod trie;
use crate::models::batch::BatchStatus;
use crate::trie_cache::batch_proof::BatchProof;
use crate::trie_cache::item::CachedItem;
use crate::{db, TrieCacheError};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use tracing::info;
use trie::Trie;
pub struct TrieCache {}

impl TrieCache {
    /// Creates a batch in the TrieCache.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to a pooled SQLite connection.
    /// * `items` - A vector of CachedItem objects.
    ///
    /// # Returns
    ///
    /// Returns a Result containing a BatchProof if successful, or a TrieCacheError if an error occurs.
    // ToDo: This should have a mutex to prevent concurrent access
    pub fn create_batch(
        conn: &PooledConnection<SqliteConnectionManager>,
        items: Vec<CachedItem>,
    ) -> Result<BatchProof, TrieCacheError> {
        match db::batch::get_latest_batch_by_status(conn, BatchStatus::Created) {
            Ok(Some(latest_batch)) => {
                let (storage, trie) = Trie::load(latest_batch.root_idx, conn);
                let batch_id = latest_batch.id + 1;
                let (batch_proof, root_idx) = Trie::persist_batch_and_generate_proofs(
                    storage,
                    trie,
                    latest_batch.root_idx,
                    items,
                    &batch_id,
                )?;
                db::batch::create_batch(conn, Some(latest_batch.id), root_idx)?;
                info!("Batch created with id: {}", batch_id);
                Ok(batch_proof)
            }
            Ok(None) => {
                let batch_id = 1;
                let (storage, trie) = Trie::new(conn);
                let (batch_proof, root_idx) =
                    Trie::persist_batch_and_generate_proofs(storage, trie, 1, items, &batch_id)?;

                db::batch::create_batch(conn, None, root_idx)?;
                Ok(batch_proof)
            }
            Err(e) => Err(e),
        }
    }

    /// Updates the status of a batch in the TrieCache.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to a pooled SQLite connection.
    /// * `batch_id` - The ID of the batch to update.
    /// * `status` - The new status of the batch.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if the update is successful, or a TrieCacheError if an error occurs.
    pub fn update_batch_status(
        conn: &PooledConnection<SqliteConnectionManager>,
        batch_id: u64,
        status: BatchStatus,
    ) -> Result<(), TrieCacheError> {
        info!("Updating batch # {:?} status to {:?}", batch_id, status);
        match status {
            BatchStatus::Finalized => {
                let batch = db::batch::get_batch(conn, batch_id)?;
                match batch.parent_id {
                    Some(parent_id) => {
                        let parent_batch = db::batch::get_batch(conn, parent_id)?;
                        match parent_batch.status {
                            BatchStatus::Finalized => {
                                db::batch::update_batch_status(
                                    conn,
                                    &batch_id,
                                    BatchStatus::Finalized,
                                )?;
                                info!("Update Complete");
                                Ok(())
                            }
                            _ => Err(TrieCacheError::BatchParentNotFinalized),
                        }
                    }
                    None => {
                        db::batch::update_batch_status(conn, &batch_id, BatchStatus::Finalized)?;
                        info!("Update Complete");
                        Ok(())
                    }
                }
            }
            _ => todo!(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_database() -> Connection {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_owned();
        let conn = SqliteConnectionManager::file(&temp_path).connect().unwrap();
        conn
    }

    #[test]
    fn test_create_batch() {
        let conn = create_temp_database();


        let result = TrieCache::create_batch(&conn, items);

        assert!(result.is_ok());
        let batch_proof = result.unwrap();
        assert_eq!(batch_proof.proofs.len(), 2);
    }

    #[test]
    fn test_update_batch_status_finalized() {
        let conn = create_temp_database();
        let batch_id = 1;
        let status = BatchStatus::Finalized;

        let result = TrieCache::update_batch_status(&conn, batch_id, status);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_batch_status_not_finalized() {
        let conn = create_temp_database();
        let batch_id = 1;
        let status = BatchStatus::Created;

        let result = TrieCache::update_batch_status(&conn, batch_id, status);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, TrieCacheError::BatchParentNotFinalized);
    }
}
