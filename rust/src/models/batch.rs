use crate::errors::TrieCacheError;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Batch {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub status: BatchStatus,
    pub root_idx: u64,
}

impl TryFrom<&Row<'_>> for Batch {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let status: String = row.get(2)?;
        Ok(Batch {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            status: BatchStatus::from_str(status.as_str()).unwrap_or(BatchStatus::Reverted),
            root_idx: row.get(3)?,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BatchStatus {
    Created = 1,
    Finalized = 2,
    Reverted = 3,
}

impl BatchStatus {
    /// Converts the `BatchStatus` enum variant to its corresponding string representation.
    ///
    /// # Returns
    ///
    /// - The string representation of the `BatchStatus` enum variant.
    pub fn to_string(&self) -> String {
        match self {
            BatchStatus::Created => "created".to_string(),
            BatchStatus::Finalized => "finalized".to_string(),
            BatchStatus::Reverted => "reverted".to_string(),
        }
    }
}

impl FromStr for BatchStatus {
    type Err = TrieCacheError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "created" => Ok(BatchStatus::Created),
            "finalized" => Ok(BatchStatus::Finalized),
            "reverted" => Ok(BatchStatus::Reverted),
            _ => Err(TrieCacheError::InvalidBatchStatus),
        }
    }
}
