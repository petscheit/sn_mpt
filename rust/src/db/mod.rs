use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod batch;
pub mod trie;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Debug)]
pub struct ConnectionManager {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl ConnectionManager {
    /// Creates a new ConnectionManager with a connection pool to the specified database file.
    pub fn new(file: &str, test_mode: bool) -> Self {
        let db_file = if test_mode {
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            let timestamp = since_the_epoch.as_secs();
            format!("{}_{}", timestamp, file)
        } else {
            file.into()
        };
        let manager = SqliteConnectionManager::file(db_file);
        let pool = Pool::new(manager).unwrap();
        ConnectionManager {
            pool: Arc::new(pool),
        }
    }

    /// Gets a connection from the pool.
    pub fn get_connection(&self) -> anyhow::Result<PooledConnection<SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }

    pub fn create_table(&self) -> anyhow::Result<()> {
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
