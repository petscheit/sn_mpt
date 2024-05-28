use pathfinder_common::hash::FeltHash;
use rusqlite::{Connection};

#[derive(Debug)]
pub struct DB {
    pub connection: Connection,
    file: String,
}

impl DB {
    pub fn new(file: String) -> anyhow::Result<DB> {
        let connection = Connection::open(file.clone())?;
        Ok(DB { connection, file })
    }

    pub fn create_table(&self) -> anyhow::Result<()> {
        self.connection.execute(
             "CREATE TABLE IF NOT EXISTS trie_nodes (
                idx INTEGER PRIMARY KEY,
                hash BLOB NOT NULL,
                data BLOB,
                trie_idx INTEGER UNIQUE NOT NULL
            )",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS leaves (
                idx INTEGER PRIMARY KEY,
                key BLOB NOT NULL,
                commitment BLOB NOT NULL,
                value BLOB,
                batch_id INTEGER NOT NULL
            )",
            [],
        )?;

        self.connection.execute(
             "CREATE TABLE IF NOT EXISTS batches (
                id INTEGER PRIMARY KEY,
                parent_id INTEGER,
                status INTEGER NOT NULL,
                root_idx INTEGER NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES batches(id)
            )",
            [],
        )?;

        Ok(())
    }
}

