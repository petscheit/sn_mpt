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
                remaining_path = &remaining_path[1..];
            }
            TrieNode::Edge { child, path } => {
                if path != &remaining_path[..path.len()] {
                    // If paths don't match, we've found a proof of non membership because
                    // we:
                    // 1. Correctly moved towards the target insofar as is possible, and
                    // 2. hashing all the nodes along the path does result in the root hash,
                    //    which means
                    // 3. the target definitely does not exist in this tree
                    return Some(Membership::NonMember);
                }

                expected_hash = *child;

                remaining_path = &remaining_path[path.len()..];
            }
        }
    }

    assert!(remaining_path.is_empty(), "Proof path should be empty");

    // At this point, we should reach `value` !
    if expected_hash == value {
        Some(Membership::Member)
    } else {
        // Hash mismatch. Return `None`.
        None
    }
}