use std::sync::Arc;

pub mod batch;
pub mod trie;

use crate::errors::TrieCacheError;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Debug)]
pub struct ConnectionManager {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl ConnectionManager {
    /// Creates a new ConnectionManager with a connection pool to the specified database file.
    pub fn new(file: &str) -> Self {
        let manager = SqliteConnectionManager::file(file);
        let pool = Pool::new(manager).unwrap();
        ConnectionManager {
            pool: Arc::new(pool),
        }
    }

    /// Gets a connection from the pool.
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, TrieCacheError> {
        Ok(self.pool.get()?)
    }

    pub fn create_table(&self) -> Result<(), TrieCacheError> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS trie_nodes (
                idx INTEGER PRIMARY KEY,
                hash BLOB NOT NULL,
                data BLOB,
                trie_idx INTEGER UNIQUE NOT NULL
            )",
            [],
        )?;

        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS leaves (
                idx INTEGER PRIMARY KEY,
                key BLOB NOT NULL,
                commitment BLOB NOT NULL,
                value BLOB,
                batch_id INTEGER NOT NULL
            )",
            [],
        )?;

        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS batches (
                id INTEGER PRIMARY KEY,
                parent_id INTEGER,
                status TEXT NOT NULL,
                root_idx INTEGER NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES batches(id)
            )",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        db::ConnectionManager,
        models::batch::{Batch, BatchStatus},
    };
    use rand::random;
    use std::{path::Path, sync::Arc};

    use super::batch::{create_batch, update_batch_status};

    pub struct TestContext {
        pub(crate) manager: Arc<ConnectionManager>,
        db_file: String,
    }

    impl TestContext {
        pub(crate) fn new() -> Self {
            let rand = random::<u32>();
            let file = format!("{:?}_{}", rand, "test.db");
            let manager = Arc::new(ConnectionManager::new(file.as_str()));
            manager.create_table().unwrap();

            TestContext {
                manager,
                db_file: file.to_string(),
            }
        }

        pub fn batch_seeding(&self) -> Vec<Batch> {
            let conn = self.manager.get_connection().unwrap();
            create_batch(&conn, None, 1).unwrap();
            create_batch(&conn, Some(1), 7).unwrap();
            create_batch(&conn, Some(2), 16).unwrap();
            update_batch_status(&conn, &1u64, BatchStatus::Finalized).unwrap();

            vec![
                Batch {
                    id: 1,
                    parent_id: None,
                    status: BatchStatus::Finalized,
                    root_idx: 1,
                },
                Batch {
                    id: 2,
                    parent_id: Some(1),
                    status: BatchStatus::Created,
                    root_idx: 7,
                },
                Batch {
                    id: 3,
                    parent_id: Some(2),
                    status: BatchStatus::Created,
                    root_idx: 16,
                },
            ]
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Delete the database file
            let _ = std::fs::remove_file(Path::new(&self.db_file));
        }
    }
}
