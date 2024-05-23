use pathfinder_common::{hash::PoseidonHash, trie::TrieNode};
use pathfinder_crypto::Felt;
use crate::{BatchUpdate, BatchUpdateCairo0, LeafUpdate, LeafUpdateCairo0};
use std::collections::HashMap;
use serde_json::to_string_pretty;

pub trait CairoCompatible {
    fn to_cairo_str(&self, in_array: bool) -> String;
}

impl CairoCompatible for LeafUpdate {
    fn to_cairo_str(&self, in_array: bool) -> String {
        let mut output = String::new();
        output += "LeafUpdate {\n";
        output += &format!("    key: 0x{},\n", hex::encode(self.key.to_be_bytes()));
        output += "    proof_pre: ";
        output += &leaf_proof_to_cairo_string(self.proof_pre.clone(), true);
        output += "    proof_post: ";
        output += &leaf_proof_to_cairo_string(self.proof_post.clone(), true);
        if in_array {
            output += "},\n";
        } else {
            output += "};\n";
        }

        output
    }
}

impl CairoCompatible for BatchUpdate {
    fn to_cairo_str(&self, _in_array: bool) -> String {
        let mut output = String::new();
        output += "BatchUpdate {\n";
        output += &format!("    pre_root: 0x{},\n", hex::encode(self.pre_root.to_be_bytes()));
        output += &format!("    post_root: 0x{},\n", hex::encode(self.post_root.to_be_bytes()));
        output += "    leaf_updates: array![\n";
        for update in &self.leaf_updates {
            output += &update.to_cairo_str(true);
        }
        output += "    ]\n";
        output += "};\n";

        output
    }
}



impl CairoCompatible for BatchUpdateCairo0 {
    fn to_cairo_str(&self, _in_array: bool) -> String {
        to_string_pretty(&self).unwrap()
    }
}

impl From<&LeafUpdate> for LeafUpdateCairo0 {
    fn from(item: &LeafUpdate) -> Self {
        LeafUpdateCairo0 {
            key: hex::encode(item.key.to_be_bytes()),
            pre_value: hex::encode(item.value_pre.to_be_bytes()),
            post_value: hex::encode(item.value_post.to_be_bytes()),
        }
    }
}

impl From<&BatchUpdate> for BatchUpdateCairo0 {
    fn from(item: &BatchUpdate) -> Self {

        let (preimage, leaf_updates) = generate_preimage_and_updates(&item.leaf_updates);
        BatchUpdateCairo0 {
            pre_root: hex::encode(item.pre_root.to_be_bytes()),
            post_root: hex::encode(item.post_root.to_be_bytes()),
            preimage,
            leaf_updates,
        }
    }
}


fn generate_preimage_and_updates(updates: &Vec<LeafUpdate>) -> (HashMap<String, Vec<String>>, Vec<LeafUpdateCairo0>) {
    fn convert(values: &[TrieNode], map: &mut HashMap<String, Vec<String>>) {
        values.iter().for_each(|node| {
            let hash = node.hash::<PoseidonHash>();
            match node {
                TrieNode::Binary { left, right } => {
                    map.insert(
                        hex::encode(hash.to_be_bytes()),
                        vec![
                            hex::encode(left.to_be_bytes()),
                            hex::encode(right.to_be_bytes()),
                        ]
                    );
                },
                TrieNode::Edge { child, path } => {
                    map.insert(
                        hex::encode(hash.to_be_bytes()),
                        vec![
                            hex::encode(path.len().to_be_bytes()),
                            hex::encode(Felt::from_bits(path).unwrap().to_be_bytes()),
                            hex::encode(child.to_be_bytes())
                        ]
                    );
                },
            }
        });
    }

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut leaf_updates = vec![];

    updates.iter().for_each(|update| {
        leaf_updates.push(update.into());
        convert(&update.proof_pre, &mut map);
        convert(&update.proof_post, &mut map);
    });

    (map, leaf_updates)
}

pub fn leaf_proof_to_cairo_string(proof: Vec<TrieNode>, in_array: bool) -> String {
    let mut output = String::new();
    output += "array![\n";

    let last_index = proof.len() - 1; // to handle the trailing comma logic
    for (index, node) in proof.iter().enumerate() {
        match node {
            TrieNode::Binary { left, right } => {
                output += &format!(
                    "        TrieNode::Binary(BinaryNodeImpl::new(0x{},0x{})){}\n",
                    hex::encode(left.to_be_bytes()),
                    hex::encode(right.to_be_bytes()),
                    if index != last_index { "," } else { "" }
                );
            },
            TrieNode::Edge { child, path } => {
                output += &format!(
                    "        TrieNode::Edge(EdgeNodeImpl::new(0x{}, 0x{}, {})){}\n",
                    hex::encode(Felt::from_bits(path).unwrap().to_be_bytes()),
                    hex::encode(child.to_be_bytes()),
                    path.len(),
                    if index != last_index { "," } else { "" }
                );
            },
        }
    }
    if in_array {
        output += "    ],\n";
    } else {
        output += "    ];\n";
    }

    output
}
