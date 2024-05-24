use pathfinder_common::hash::{FeltHash, PoseidonHash};
mod batch_proof;
mod batcher;
mod items;
mod persistance;
mod tree;

use crate::batcher::Batcher;
use crate::items::CachedItem;

struct HdpCache<H: FeltHash, const HEIGHT: usize> {
    batcher: Batcher<H, HEIGHT>,
}

trait Cache {
    fn new() -> Self;
    fn init_from_storage() -> Self;
    fn submit(&mut self, items: Vec<CachedItem>) -> anyhow::Result<()>;
    fn finalize(&mut self, batch_id: u64) -> anyhow::Result<()>;
}

impl<H: FeltHash, const HEIGHT: usize> Cache for HdpCache<H, HEIGHT> {
    fn new() -> Self {
        Self {
            batcher: Batcher::<H, HEIGHT>::new(),
        }
    }

    fn submit(&mut self, items: Vec<CachedItem>) -> anyhow::Result<()> {
        let (batch_id, batch_proof) = self.batcher.add_batch(items)?;
        println!("Batch ID: {:?}", batch_id);
        println!("Batch Proof: {:?}", batch_proof);
        Ok(())
    }

    fn finalize(&mut self, batch_id: u64) -> anyhow::Result<()> {
        self.batcher.finalize_batch(batch_id)
    }

    fn init_from_storage() -> Self {
        todo!()
    }
}

fn main() {
    let mut cache = HdpCache::<PoseidonHash, 251>::new();

    let items = vec![
        CachedItem::new(vec![1, 2, 3]),
        CachedItem::new(vec![4, 5, 6]),
    ];
    let res = cache.submit(items);
    let items = vec![
        CachedItem::new(vec![7, 8, 9]),
        CachedItem::new(vec![10, 11, 12]),
    ];
    let _ = cache.submit(items);

    cache.finalize(1).unwrap()
}
