use pathfinder_crypto::{Felt, MontFelt};
use pathfinder_crypto::hash::{poseidon_hash_many, poseidon_hash};
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub struct CachedItem {
    pub value: Vec<u8>,
    pub key: Felt,
    pub commitment: Felt,
}

impl CachedItem {
    pub fn new(value: Vec<u8>) -> Self {
        let commitment = Self::commitment(&value);
        let key = Self::gen_key(&commitment);
        Self {
            value,
            key,
            commitment,
        }
    }

    fn commitment(value: &Vec<u8>) -> Felt {
        poseidon_hash_many(&*vec_to_mont_felts(&value)).into()
    }

    fn gen_key(commitment: &Felt) -> Felt {
        poseidon_hash(commitment.clone().into(), commitment.clone().into()).into()
    }
}


fn vec_to_mont_felts(data: &Vec<u8>) -> Vec<MontFelt> {
    const CHUNK_SIZE: usize = 32;
    let mut mont_felts = Vec::with_capacity((data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE);
    for chunk in data.chunks(CHUNK_SIZE) {
        let mut buffer = [0u8; CHUNK_SIZE];
        buffer[..chunk.len()].copy_from_slice(chunk);
        mont_felts.push(MontFelt::from_be_bytes(buffer));
    }

    mont_felts
}

#[cfg(test)]
impl Default for CachedItem {
    fn default() -> Self {
        let mut rng = thread_rng();
        CachedItem::new((0..10).map(|_| rng.gen()).collect())
    }
}