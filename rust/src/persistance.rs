use anyhow::Context;
use bitvec::prelude::{BitSlice, Msb0};
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_storage::StoredNode;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Persistance {
    pub nodes: HashMap<u64, (Felt, StoredNode)>,
    pub leaves: HashMap<Felt, Felt>,
    pub next_index: u64,
}
impl Storage for Persistance {
    fn get(&self, index: u64) -> anyhow::Result<Option<StoredNode>> {
        Ok(self.nodes.get(&index).map(|x| x.1.clone()))
    }

    fn hash(&self, index: u64) -> anyhow::Result<Option<Felt>> {
        Ok(self.nodes.get(&index).map(|x| x.0))
    }

    fn leaf(&self, path: &BitSlice<u8, Msb0>) -> anyhow::Result<Option<Felt>> {
        let key = Felt::from_bits(path).context("Mapping path to felt")?;

        Ok(self.leaves.get(&key).cloned())
    }
}
