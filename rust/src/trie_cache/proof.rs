use bitvec::prelude::Msb0;
use bitvec::slice::BitSlice;
use pathfinder_common::hash::PedersenHash;
use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::merkle_node::Direction;

#[derive(Debug)]
pub enum Membership {
    Member,
    NonMember,
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
        if proof_node.hash::<PedersenHash>() != expected_hash {
            return None;
        }
        match proof_node {
            TrieNode::Binary { left, right } => {
                let direction = Direction::from(remaining_path[0]);
                expected_hash = match direction {
                    Direction::Left => *left,
                    Direction::Right => *right,
                };
                remaining_path = &remaining_path[1..];
            }
            TrieNode::Edge { child, path } => {
                if path != &remaining_path[..path.len()] {
                    return Some(Membership::NonMember);
                }

                expected_hash = *child;

                remaining_path = &remaining_path[path.len()..];
            }
        }
    }

    assert!(remaining_path.is_empty(), "Proof path should be empty");

    if expected_hash == value {
        Some(Membership::Member)
    } else {
        None
    }
}