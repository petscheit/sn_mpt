use std::cmp::PartialEq;
use std::collections::HashMap;
use anyhow::anyhow;
use pathfinder_common::hash::FeltHash;
use crate::batch_proof::BatchProof;
use crate::items::CachedItem;
use crate::tree::CacheTree;

pub struct Batcher<H: FeltHash, const HEIGHT: usize> {
    finalized: u64,
    latest: u64,
    batches: HashMap<u64, Batch>,
    tree: CacheTree<H, HEIGHT>,
}

impl<H: FeltHash, const HEIGHT: usize> Batcher<H, HEIGHT> {
    pub fn new() -> Self {
        Self {
            finalized: 0,
            latest: 0,
            batches: HashMap::new(),
            tree: CacheTree::<H, HEIGHT>::new(),
        }
    }

    pub fn add_batch(&mut self, items: Vec<CachedItem>) -> anyhow::Result<(u64, BatchProof)> {
        let id = self.latest + 1;

        let batch_proof = self.tree.commit_batch(items, &id)?;

        let batch = Batch {
            id,
            parent_id: if self.latest != 0 { Some(self.latest) } else { None },
            status: BatchStatus::Proving,
        };

        self.latest = id;
        self.batches.insert(id, batch);

        Ok((id, batch_proof))
    }

   pub fn finalize_batch(&mut self, batch_id: u64) -> anyhow::Result<()> {
        let batch = self.batches.get(&batch_id)
            .ok_or_else(|| anyhow!("Batch not found"))?;

        if let Some(parent_id) = batch.parent_id {
            let parent = self.batches.get(&parent_id)
                .ok_or_else(|| anyhow!("Parent batch not found"))?;
            if parent.status != BatchStatus::Finalized {
                return Err(anyhow!("Parent batch not finalized"));
            }
        }

        if let Some(batch) = self.batches.get_mut(&batch_id) {
            batch.update_status(BatchStatus::Finalized);
            self.finalized = batch_id;
            Ok(())
        } else {
            Err(anyhow!("Batch not found"))  // This line is mostly redundant but added for completeness
        }
   }
}

#[derive(Debug)]
pub struct Batch {
    id: u64,
    parent_id: Option<u64>, // id of the parent batch
    status: BatchStatus // status of batch
}

impl Batch {
    pub fn update_status(&mut self, status: BatchStatus) {
        self.status = status;
    }
}

#[derive(Debug, PartialEq)]
enum BatchStatus {
    Scheduled,
    Proving,
    Ready,
    Finalized,
    Reverted,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::CachedItem;
    use pathfinder_common::hash::PoseidonHash;

    #[test]
    fn test_add_batch() {
        let mut batcher = Batcher::<PoseidonHash, 10>::new();
        let items: Vec<_> = (0..10).map(|_|CachedItem::default()).collect();
        assert_eq!(batcher.latest, 0);
        assert_eq!(batcher.finalized, 0);

        assert!(batcher.add_batch(items).is_ok());
        assert_eq!(batcher.latest, 1);
        assert_eq!(batcher.finalized, 0);
        assert_eq!(batcher.batches.len(), 1);
    }

    #[test]
    fn test_finalize_batch() {
        let mut batcher = Batcher::<PoseidonHash, 10>::new();
        let items: Vec<_> = (0..10).map(|_|CachedItem::default()).collect();

        assert!(batcher.add_batch(items).is_ok());
        assert!(batcher.finalize_batch(1).is_ok());
        assert_eq!(batcher.finalized, 1);
        assert_eq!(batcher.latest, 1);
        assert_eq!(batcher.batches.get(&1).unwrap().status, BatchStatus::Finalized);
    }

    #[test]
    fn test_parent_finalization_guard() {
        let mut batcher = Batcher::<PoseidonHash, 10>::new();
        let items: Vec<_> = (0..10).map(|_|CachedItem::default()).collect();

        assert!(batcher.add_batch(items.clone()).is_ok());
        assert!(batcher.add_batch(items).is_ok());

        assert!(!batcher.finalize_batch(2).is_ok());

        assert!(batcher.finalize_batch(1).is_ok());
        assert!(batcher.finalize_batch(2).is_ok());
    }
}