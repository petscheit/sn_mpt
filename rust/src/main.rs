use pathfinder_common::hash::{FeltHash, PoseidonHash};
use crate::batch_proof::BatchProof;

mod batch_proof;
mod items;
mod persistance;
mod tree;
mod db;
mod cache;

use crate::cache::CacheStore;
use crate::items::CachedItem;

struct HdpCache<H: FeltHash, const HEIGHT: usize> {
    cache: CacheStore<H, HEIGHT>,
}

trait Cache {
    fn new() -> Self;
    fn init_from_storage() -> Self;
    fn submit(&mut self, items: Vec<CachedItem>) -> anyhow::Result<BatchProof>;
    fn finalize(&mut self, batch_id: u64) -> anyhow::Result<()>;
}

impl<H: FeltHash, const HEIGHT: usize> Cache for HdpCache<H, HEIGHT> {
    fn new() -> Self {
        Self {
            cache: CacheStore::<H, HEIGHT>::new(),
        }
    }

    fn submit(&mut self, items: Vec<CachedItem>) -> anyhow::Result<BatchProof> {
        self.cache.create_batch(items)
    }

    fn finalize(&mut self, batch_id: u64) -> anyhow::Result<()> {
        self.cache.finalize_batch(batch_id)
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
    let res = cache.submit(items).unwrap();
    println!("{:?}", res);
    let items = vec![
        CachedItem::new(vec![7, 8, 9]),
        CachedItem::new(vec![10, 11, 12]),
    ];
    let res = cache.submit(items);
    println!("{:?}", res);

    println!("{:?}", cache.finalize(1));
    println!("{:?}", cache.finalize(2));
}
