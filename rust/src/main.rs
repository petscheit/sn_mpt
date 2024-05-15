use std::collections::HashMap;
use anyhow::Context;
use bitvec::prelude::{BitSlice, BitVec, Msb0};
use pathfinder_merkle_tree::{tree::MerkleTree, storage::Storage};
use pathfinder_common::hash::{FeltHash, PedersenHash};
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_common::felt;
use bitvec::view::BitView;
use pathfinder_storage::{Node, NodeRef, StoredNode};
use pathfinder_merkle_tree::merkle_node::{Direction};

use hex; 
struct TestTrie {
    storage: TestStorage,
    tree: MerkleTree<PedersenHash, 251>,
}

#[derive(Debug)]
pub enum Membership {
    Member,
    NonMember,
}

fn main() {

    let mut trie = TestTrie::new();


    let mut i: u64 = 0;
    while i < 2000 {
        let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(i));
        let value = PedersenHash::hash(key.clone(), key.clone());
        trie.tree.set(&trie.storage, key.view_bits().to_bitvec(), value).unwrap();
        i += 1;
    }

    let (commitment, index) = commit_and_persist(trie.tree.clone(), &mut trie.storage, true);
    let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(30));
    let value = PedersenHash::hash(key.clone(), key.clone());
    // println!("Value: {:?}", value);
    let mut proof = MerkleTree::<PedersenHash, 251>::get_proof(index, &trie.storage, &*key.view_bits().to_bitvec()).unwrap().unwrap();

    // // println!("Proof: {:?}", proof);
    // let new_proof = &proof[..12];
    // println!("New Proof: {:?}", new_proof);
    //
    let res = verify_proof(commitment, &*key.view_bits().to_bitvec(), value, proof.as_ref());
    //
    println!("Proof: {:?}", res);

    print_cairo_proof(commitment, key, proof.clone());
}

// fn main() {
//
//     let mut trie = TestTrie::new();
//
//     // Seed and persist tree
//     let mut i: u64 = 0;
//     while i < 2000 {
//         let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(i));
//         let value = PedersenHash::hash(key.clone(), key.clone());
//         trie.tree.set(&trie.storage, key.view_bits().to_bitvec(), value).unwrap();
//         i += 1;
//     }
//     let (commitment, index) = commit_and_persist(trie.tree.clone(), &mut trie.storage, true);
//
//     let key = PedersenHash::hash(felt!("0x0"), Felt::from_u64(7777));
//     let value = PedersenHash::hash(key.clone(), Felt::from_u64(3));
//
//     // Generate pre-update proof
//     let proof_pre = MerkleTree::<PedersenHash, 251>::get_proof(index, &trie.storage, &*key.clone().view_bits().to_bitvec()).unwrap().unwrap();
//     println!("PreProof");
//     print_cairo_proof(commitment, key, proof_pre.clone());
//
//     // Update and persist tree
//     trie.tree.set(&trie.storage, key.clone().view_bits().to_bitvec(), value).unwrap();
//     let (commitment, index) = commit_and_persist(trie.tree.clone(), &mut trie.storage, true);
//
//     // Generate post-update proof
//     let proof_post = MerkleTree::<PedersenHash, 251>::get_proof(index, &trie.storage, &*key.clone().view_bits().to_bitvec()).unwrap().unwrap();
//     println!("PostProof");
//     print_cairo_proof(commitment, key, proof_post.clone());
//
//     // let (commitment, index) = commit_and_persist(trie.tree.clone(), &mut trie.storage, true);
// }

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
                    //bitvec_to_hex(path.clone()), Toggle these
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

fn bitvec_to_hex(bits: BitVec<u8, Msb0>) -> String {
    let mut bool = vec![];
    for bit in bits.iter() {
        bool.push(if *bit { true } else { false });
    }
    println!("Bool: {:?}", bool);
    // Convert Vec<bool> to Vec<u8> to represent each byte
    let mut result: Vec<u8> = Vec::new();
    let mut byte = 0;
    let mut bit_index = 0;

     for (index, &bit) in bool.iter().enumerate() {
        if bit {
            byte |= 1 << bit_index;
        } else {
            byte |= 0 << bit_index;
        }
        bit_index += 1;

        // Every 8 bits or at the end of the input vector, push the byte to the result vector
        // and reset for the next byte
        if bit_index == 8 || index == bool.len() - 1 {
            result.push(byte);
            byte = 0;
            bit_index = 0;
        }
    }


    // Convert bytes to a hexadecimal string
    let hex_string: Vec<String> = result.iter().map(|&b| format!("{:02X}", b)).collect();
    println!("Hex: {:?}", hex_string);
    hex_string.join("")

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

 fn verify_proof(
    root: Felt,
    key: &BitSlice<u8, Msb0>,
    value: Felt,
    proofs: &[TrieNode],
) -> Option<Membership> {
    // Protect from ill-formed keys
    if key.len() != 251 {
        return None;
    }

    let mut expected_hash = root;
    let mut remaining_path: &BitSlice<u8, Msb0> = key;

    for proof_node in proofs.iter() {
        // Hash mismatch? Return None.
        if proof_node.hash::<PedersenHash>() != expected_hash {
            return None;
        }
        match proof_node {
            TrieNode::Binary { left, right } => {
                // Direction will always correspond to the 0th index
                // because we're removing bits on every iteration.
                let direction = Direction::from(remaining_path[0]);

                // Set the next hash to be the left or right hash,
                // depending on the direction
                expected_hash = match direction {
                    Direction::Left => *left,
                    Direction::Right => *right,
                };

                // Advance by a single bit
                remaining_path = &remaining_path[1..];
            }
            TrieNode::Edge { child, path } => {
                // println!("path_len: {}", path.len());
                // println!("Path: {}",  hex::encode(path.as_raw_slice()));
                // println!("Rema: {}",  hex::encode(&remaining_path[..path.len()].to_bitvec().as_raw_slice()));
                println!("Edge Node: {:?}", proof_node);
                println!("Path: {:?}", path.clone());
                println!("Path: {:?}", path.clone().into_vec());

                println!("Edge hash: {}", proof_node.hash::<PedersenHash>());
                if path != &remaining_path[..path.len()] {
                    // If paths don't match, we've found a proof of non membership because
                    // we:
                    // 1. Correctly moved towards the target insofar as is possible, and
                    // 2. hashing all the nodes along the path does result in the root hash,
                    //    which means
                    // 3. the target definitely does not exist in this tree
                    return Some(Membership::NonMember);
                }

                // Set the next hash to the child's hash
                expected_hash = *child;

                // Advance by the whole edge path
                remaining_path = &remaining_path[path.len()..];
            }
        }
    }

    // At this point, we should reach `value` !
    if expected_hash == value {
        Some(Membership::Member)
    } else {
        // Hash mismatch. Return `None`.
        None
    }
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
