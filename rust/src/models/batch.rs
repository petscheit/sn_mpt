use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub status: BatchStatus,
    pub root_idx: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BatchStatus {
    Created = 1,
    Finalized = 2,
    Reverted = 3,
}

impl BatchStatus {
    pub fn to_string(&self) -> String {
        match self {
            BatchStatus::Created => "created".to_string(),
            BatchStatus::Finalized => "finalized".to_string(),
            BatchStatus::Reverted => "reverted".to_string(),
        }
    }
}

impl FromStr for BatchStatus {
    type Err = warp::Rejection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "created" => Ok(BatchStatus::Created),
            "finalized" => Ok(BatchStatus::Finalized),
            "reverted" => Ok(BatchStatus::Reverted),
            _ => Err(warp::reject::not_found()),
        }
    }
}