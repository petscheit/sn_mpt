use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use std::str::FromStr;

use crate::models::batch::{Batch, BatchStatus};
use crate::TrieCacheError;

pub fn get_batches(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<Vec<Batch>, TrieCacheError> {
    let mut stmt = conn.prepare_cached("SELECT id, parent_id, status, root_idx FROM batches")?;
    let batches_tupel = stmt.query_map([], |row| {
        let id: u64 = row.get(0)?;
        let parent_id: Option<u64> = row.get(1)?;
        let status_str: String = row.get(2)?;
        let root_idx: u64 = row.get(3)?;

        Ok((id, parent_id, status_str, root_idx))
    })?;

    let mut batches = Vec::new();
    for batch in batches_tupel {
        match batch {
            Ok((id, parent_id, status, root_idx)) => {
                batches.push(Batch {
                    id,
                    parent_id,
                    status: BatchStatus::from_str(status.as_str())?,
                    root_idx,
                });
            }
            Err(e) => return Err(TrieCacheError::from(e)),
        }
    }

    Ok(batches)
}

pub fn get_batch(
    conn: &PooledConnection<SqliteConnectionManager>,
    id: u64,
) -> Result<Batch, TrieCacheError> {
    let mut stmt =
        conn.prepare_cached("SELECT id, parent_id, status, root_idx FROM batches WHERE id = ?")?;

    let (id, parent_id, status, root_idx) = stmt.query_row(params![id], |row| {
        let id: u64 = row.get(0)?;
        let parent_id: Option<u64> = row.get(1)?;
        let status_str: String = row.get(2)?;
        let root_idx: u64 = row.get(3)?;

        Ok((id, parent_id, status_str, root_idx))
    })?;

    Ok(Batch {
        id,
        parent_id,
        status: BatchStatus::from_str(status.as_str())?,
        root_idx,
    })
}

pub fn create_batch(
    conn: &PooledConnection<SqliteConnectionManager>,
    parent_id: Option<u64>,
    root_idx: u64,
) -> Result<u64, TrieCacheError> {
    const INSERT_QUERY: &str = "INSERT INTO batches (parent_id, status, root_idx) VALUES (?, ?, ?)";

    conn.execute(
        INSERT_QUERY,
        params![parent_id, BatchStatus::Created.to_string(), root_idx],
    )
    .map_err(TrieCacheError::from)?;

    // Retrieve the ID of the last inserted row
    Ok(conn.last_insert_rowid() as u64)
}

pub fn get_latest_batch_by_status(
    conn: &PooledConnection<SqliteConnectionManager>,
    status: BatchStatus,
) -> Result<Option<Batch>, TrieCacheError> {
    let mut stmt = conn.prepare_cached(
    "SELECT id, parent_id, status, root_idx FROM batches WHERE status = ? ORDER BY id DESC LIMIT 1"
    ).map_err(TrieCacheError::from)?;

    // Execute the query and attempt to fetch the row
    let vals = stmt
        .query_row(params![status.to_string()], |row| {
            let id: u64 = row.get(0)?;
            let parent_id: Option<u64> = row.get(1)?;
            let status_str: String = row.get(2)?;
            let root_idx: u64 = row.get(3)?;

            // Return all values as a tuple
            Ok((id, parent_id, status_str, root_idx))
        })
        .optional()
        .map_err(TrieCacheError::from)?;

    match vals {
        None => Ok(None),
        Some((id, parent_id, status, root_idx)) => Ok(Some(Batch {
            id,
            parent_id,
            status: BatchStatus::from_str(status.as_str())?,
            root_idx,
        })),
    }
}

pub fn update_batch_status(
    conn: &PooledConnection<SqliteConnectionManager>,
    id: &u64,
    new_status: BatchStatus,
) -> Result<(), TrieCacheError> {
    let updated_rows = conn.execute(
        "UPDATE batches SET status = ?1 WHERE id = ?2",
        params![new_status.to_string(), id],
    )?;

    if updated_rows == 0 {
        Err(TrieCacheError::BatchNotFound)
    } else {
        Ok(())
    }
}
