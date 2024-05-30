pub mod batch_proof;
pub mod item;
pub mod trie;
use crate::models::batch::BatchStatus;
use crate::trie_cache::batch_proof::BatchProof;
use crate::trie_cache::item::CachedItem;
use crate::{db, errors::TrieCacheError};
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
mod test {
    use super::*;

    #[test]
    fn test_batch() {
        let test_ctx = db::test::TestContext::new();
        let conn = test_ctx.manager.get_connection().unwrap();

        let items: Vec<_> = (0..10).map(|_| CachedItem::default()).collect();
        let result = TrieCache::create_batch(&conn, items).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(
            result.pre_root,
            "07020e0f5c03f535f90ed3789c7f6e1aaa9694328a00a48a349d65d2f9870e72"
        );
        assert_eq!(
            result.post_root,
            "01737140f9fb422940bf92518c92455d6c08df6ef8f02333eec546512dd69ec4"
        );

        let items_two = (0..10).map(|_| CachedItem::default()).collect();
        let result_two = TrieCache::create_batch(&conn, items_two).unwrap();
        assert_eq!(result_two.id, 2);
        assert_eq!(result.post_root, result_two.pre_root);

        // Parent not finalized
        assert!(TrieCache::update_batch_status(&conn, 2, BatchStatus::Finalized).is_err());

        // Finalize parent
        assert!(TrieCache::update_batch_status(&conn, 1, BatchStatus::Finalized).is_ok());

        // Finalize child
        assert!(TrieCache::update_batch_status(&conn, 2, BatchStatus::Finalized).is_ok());
    }
}
