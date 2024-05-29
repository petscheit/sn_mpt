use crate::batch_proof::BatchProof;
use crate::items::CachedItem;
use crate::tree::CacheTree;
use anyhow::anyhow;
use pathfinder_common::hash::FeltHash;
use std::cmp::PartialEq;
use crate::persistance::{CacheDB, Persistance};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
pub struct CacheStore<H: FeltHash, const HEIGHT: usize> {
    pub persistance: Persistance,
    tree: CacheTree<H, HEIGHT>,
}

impl<H: FeltHash, const HEIGHT: usize> CacheStore<H, HEIGHT> {
    pub fn new() -> Self {
        Self {
            tree: CacheTree::<H, HEIGHT>::new(),
            persistance: Persistance::new(),
        }
    }

    pub fn create_batch(&mut self, items: Vec<CachedItem>) -> anyhow::Result<BatchProof> {
        match self.persistance.get_latest_batch() {
            Ok(Some(latest_batch)) => {
                let batch_id = latest_batch.id + 1;
                let (batch_proof, root_idx) = self.tree.commit_batch(items, &batch_id)?;
                self.persistance.create_batch(Some(latest_batch.id), root_idx)?;
                Ok(batch_proof)
            }
            Ok(None) => {
                let batch_id = 1;
                let (batch_proof, root_idx) = self.tree.commit_batch(items, &batch_id)?;
                self.persistance.create_batch(None, root_idx)?;
                Ok(batch_proof)
            }
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn finalize_batch(&mut self, batch_id: u64) -> anyhow::Result<()> {
        let batch = self.persistance.get_batch(batch_id)?;
            match batch.parent_id {
            Some(parent_id) => {
                let parent_batch = self.persistance.get_batch(parent_id)?;
                match parent_batch.status {
                    BatchStatus::Finalized => {
                        self.tree.finalize_batch(&batch_id)?;
                        Ok(())
                    }
                    _ => Err(anyhow!("Parent batch not finalized")),
                }
            }
            None => {
                self.tree.finalize_batch(&batch_id)?;
                Ok(())
            }
        }
    }
}


#[derive(Debug)]
pub struct Batch {
    pub(crate) id: u64,
    pub(crate) parent_id: Option<u64>, // id of the parent batch
    pub(crate) status: BatchStatus,    // status of batch
    pub(crate) root_idx: u64,          // index of the root node in the trie
}

impl Batch {
    pub fn update_status(&mut self, status: BatchStatus) {
        self.status = status;
    }
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BatchStatus {
    Created = 1,
    Finalized = 2,
    Reverted = 3,
}