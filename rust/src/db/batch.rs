use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use std::str::FromStr;

use crate::models::batch::{Batch, BatchStatus};
use crate::TrieCacheError;

/// Retrieves all batches from the database.
///
/// # Arguments
///
/// * `conn` - A pooled connection to the SQLite database.
///
/// # Returns
///
/// A `Result` containing a vector of `Batch` objects or a `TrieCacheError` if an error occurs.
pub fn get_batches(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<Vec<Batch>, TrieCacheError> {
    // Prepare the SQL statement
    let mut stmt = conn.prepare_cached("SELECT id, parent_id, status, root_idx FROM batches")?;

    // Execute the query and map the result rows to Batch objects
    let batches: Vec<Batch> = stmt.query_map([], |row| {
        Batch::try_from(row)
    })?.into_iter().collect::<Result<_, _>>()?;

    Ok(batches)
}

/// Retrieves a single batch from the database by its ID.
///
/// # Arguments
///
/// * `conn` - A pooled connection to the SQLite database.
/// * `id` - The ID of the batch to retrieve.
///
/// # Returns
///
/// A `Result` containing the retrieved `Batch` object or a `TrieCacheError` if an error occurs.
pub fn get_batch(
    conn: &PooledConnection<SqliteConnectionManager>,
    id: u64,
) -> Result<Batch, TrieCacheError> {
    // Prepare the SQL statement
    let mut stmt =
        conn.prepare_cached("SELECT id, parent_id, status, root_idx FROM batches WHERE id = ?")?;

    // Execute the query and retrieve the result row
    Ok(stmt.query_row(params![id], |row| {
        Batch::try_from(row)
    })?)
}

/// Creates a new batch in the database.
///
/// # Arguments
///
/// * `conn` - A pooled connection to the SQLite database.
/// * `parent_id` - The ID of the parent batch, or `None` if it has no parent.
/// * `root_idx` - The root index of the batch.
///
/// # Returns
///
/// A `Result` containing the ID of the newly created batch or a `TrieCacheError` if an error occurs.
pub fn create_batch(
    conn: &PooledConnection<SqliteConnectionManager>,
    parent_id: Option<u64>,
    root_idx: u64,
) -> Result<u64, TrieCacheError> {
    const INSERT_QUERY: &str = "INSERT INTO batches (parent_id, status, root_idx) VALUES (?, ?, ?)";

    // Execute the INSERT query
    conn.execute(
        INSERT_QUERY,
        params![parent_id, BatchStatus::Created.to_string(), root_idx],
    )
    .map_err(TrieCacheError::from)?;

    // Retrieve the ID of the last inserted row
    Ok(conn.last_insert_rowid() as u64)
}

/// Retrieves the latest batch with a specific status from the database.
///
/// # Arguments
///
/// * `conn` - A pooled connection to the SQLite database.
/// * `status` - The status of the batch to retrieve.
///
/// # Returns
///
/// A `Result` containing an `Option` of the retrieved `Batch` object or a `TrieCacheError` if an error occurs.
pub fn get_latest_batch_by_status(
    conn: &PooledConnection<SqliteConnectionManager>,
    status: BatchStatus,
) -> Result<Option<Batch>, TrieCacheError> {
    // Prepare the SQL statement
    let mut stmt = conn.prepare_cached(
    "SELECT id, parent_id, status, root_idx FROM batches WHERE status = ? ORDER BY id DESC LIMIT 1"
    ).map_err(TrieCacheError::from)?;

    Ok(stmt.query_row(params![status.to_string()], |row| {
        Batch::try_from(row)
    }).optional()?)

}

/// Updates the status of a batch in the database.
///
/// # Arguments
///
/// * `conn` - A pooled connection to the SQLite database.
/// * `id` - The ID of the batch to update.
/// * `new_status` - The new status of the batch.
///
/// # Returns
///
/// A `Result` indicating success or a `TrieCacheError` if the batch is not found.
pub fn update_batch_status(
    conn: &PooledConnection<SqliteConnectionManager>,
    id: &u64,
    new_status: BatchStatus,
) -> Result<(), TrieCacheError> {
    // Execute the UPDATE query
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
