use std::str::FromStr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use num_traits::FromPrimitive;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, OptionalExtension, params};
use crate::db::ConnectionManager;
use crate::models::batch::{Batch, BatchStatus};

pub fn get_batches(conn: &PooledConnection<SqliteConnectionManager>) -> anyhow::Result<Vec<Batch>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, parent_id, status, root_idx FROM batches",
    )?;
    let batch_iter = stmt.query_map([], |row| {
        let status_str: String = row.get(2)?;
        let status = BatchStatus::from_str(&status_str).unwrap();
        Ok(Batch {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            status,
            root_idx: row.get(3)?,
        })
    })?;

    let mut batches = Vec::new();
    for batch in batch_iter {
        batches.push(batch?);
    }

    Ok(batches)
}

pub fn get_batch(conn: &PooledConnection<SqliteConnectionManager>, id: u64) -> anyhow::Result<Batch> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, parent_id, status, root_idx FROM batches WHERE id = ?",
    )?;
    let batch = stmt.query_row(params![id], |row| {
        let status_str: String = row.get(2)?;
        let status = BatchStatus::from_str(&status_str).unwrap();
        Ok(Batch {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            status,
            root_idx: row.get(3)?,
        })
    })?;
    Ok(batch)
}

pub fn create_batch(conn: &PooledConnection<SqliteConnectionManager>, parent_id: Option<u64>, root_idx: u64) -> anyhow::Result<u64> {
    const INSERT_QUERY: &str = "INSERT INTO batches (parent_id, status, root_idx) VALUES (?, ?, ?)";
    conn.execute(
        INSERT_QUERY,
        params![parent_id, BatchStatus::Created.to_string(), root_idx],
    )?;

    Ok(conn.last_insert_rowid() as u64)
}

pub fn get_latest_batch_by_status(conn: &PooledConnection<SqliteConnectionManager>, status: BatchStatus) -> anyhow::Result<Option<Batch>> {
    let mut stmt = conn.prepare_cached(
    "SELECT id, parent_id, status, root_idx FROM batches WHERE status = ? ORDER BY id DESC LIMIT 1"
    )?;

    // Execute the query and attempt to fetch the row
    let batch = stmt.query_row(params![status.to_string()], |row| {
        let status_str: String = row.get(2)?;
        let status = BatchStatus::from_str(&status_str).unwrap();
        Ok(Batch {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            status,
            root_idx: row.get(3)?,
        })
    }).optional()?; // Use optional to handle the case where no rows are found

    Ok(batch)
}

pub fn update_batch_status(conn: &PooledConnection<SqliteConnectionManager>, id: &u64, new_status: BatchStatus) -> anyhow::Result<()> {
    let updated_rows = conn.execute(
        "UPDATE batches SET status = ?1 WHERE id = ?2",
        params![new_status.to_string(), id],
    )?;

    if updated_rows == 0 {
        Err(anyhow!("No rows were updated"))
    } else {
        Ok(())
    }
}

