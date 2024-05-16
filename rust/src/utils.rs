use pathfinder_common::trie::TrieNode;
use pathfinder_crypto::Felt;
use crate::{BatchUpdate, LeafUpdate};

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