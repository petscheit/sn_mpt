use std::collections::HashMap;
use anyhow::Context;
use bitvec::prelude::{BitSlice, Msb0};
use pathfinder_merkle_tree::{tree::MerkleTree, storage::Storage};
use pathfinder_common::hash::{FeltHash, PedersenHash};
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_common::felt;
use bitvec::view::BitView;
use pathfinder_storage::{Node, NodeRef, StoredNode};
use hex; 
struct TestTrie {
    storage: TestStorage,
    tree: MerkleTree<PedersenHash, 251>,
}

fn main() {

    let mut trie = TestTrie::new();

    let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(3));

    let mut i: u64 = 0;
    while i < 2000 {
        let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(i));
        let value = PedersenHash::hash(key.clone(), key.clone());
        trie.tree.set(&trie.storage, key.view_bits().to_bitvec(), value).unwrap();
        i += 1;
    }

    let (commitment, index) = commit_and_persist(trie.tree.clone(), &mut trie.storage, true);
    let proof = MerkleTree::<PedersenHash, 251>::get_proof(index, &trie.storage, &*key.view_bits().to_bitvec()).unwrap().unwrap();

    print_cairo_proof(commitment, key, proof.clone());

}

fn print_cairo_proof(root: Felt, key: Felt, proof: Vec<TrieNode>) {
    println!("let root = 0x{};", hex::encode(root.to_be_bytes()));
    println!("let key = 0x{};", hex::encode(key.to_be_bytes()));
    println!("let proof = array![");
    let last_index = proof.len() - 1; // to handle the trailing comma logic
    for (index, node) in proof.iter().enumerate() {
        match node {
            TrieNode::Binary { left, right } => {
                println!(
                    "    TrieNode::Binary(BinaryNodeImpl::new(0x{},0x{})){}",
                    hex::encode(left.to_be_bytes()),
                    hex::encode(right.to_be_bytes()),
                    if index != last_index { "," } else { "" }
                );
            },
            TrieNode::Edge { child, path } => {

                println!(
                    "    TrieNode::Edge(EdgeNodeImpl::new(0x{}, 0x{}, {})){}",
                    hex::encode(path.clone().into_vec()),
                    hex::encode(child.to_be_bytes()),
                    path.len(),
                    if index != last_index { "," } else { "" }
                );
            },
            _ => panic!("Unexpected node type"),
        }
    }
    println!("];");
}

 /// Commits the tree changes and persists them to storage.
fn commit_and_persist<H: FeltHash, const HEIGHT: usize>(
    tree: MerkleTree<H, HEIGHT>,
    storage: &mut TestStorage,
    prune_nodes: bool,
) -> (Felt, u64) {
    for (key, value) in &tree.leaves {
        let key = Felt::from_bits(key).unwrap();
        storage.leaves.insert(key, *value);
    }

    let update = tree.commit(storage).unwrap();

    if prune_nodes {
        for idx in update.nodes_removed {
            storage.nodes.remove(&idx);
        }
    }

    let number_of_nodes_added = update.nodes_added.len() as u64;

    for (rel_index, (hash, node)) in update.nodes_added.into_iter().enumerate() {
        let node = match node {
            Node::Binary { left, right } => {
                let left = match left {
                    NodeRef::StorageIndex(idx) => idx,
                    NodeRef::Index(idx) => storage.next_index + (idx as u64),
                };

                let right = match right {
                    NodeRef::StorageIndex(idx) => idx,
                    NodeRef::Index(idx) => storage.next_index + (idx as u64),
                };

                StoredNode::Binary { left, right }
            }
            Node::Edge { child, path } => {
                let child = match child {
                    NodeRef::StorageIndex(idx) => idx,
                    NodeRef::Index(idx) => storage.next_index + (idx as u64),
                };

                StoredNode::Edge { child, path }
            }
            Node::LeafBinary => StoredNode::LeafBinary,
            Node::LeafEdge { path } => StoredNode::LeafEdge { path },
        };

        let index = storage.next_index + (rel_index as u64);

        storage.nodes.insert(index, (hash, node));
    }

    let storage_root_index = storage.next_index + number_of_nodes_added - 1;
    storage.next_index += number_of_nodes_added;

    (update.root_commitment, storage_root_index)
}

impl TestTrie {
    pub fn new() -> Self {
        let tree = MerkleTree::<PedersenHash, 251>::empty();

        Self {  tree, storage: TestStorage::default() }
    }
}

#[derive(Default, Debug)]
struct TestStorage {
    nodes: HashMap<u64, (Felt, StoredNode)>,
    leaves: HashMap<Felt, Felt>,
    next_index: u64,
}

impl Storage for TestStorage {
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
