use crate::tree::Tree;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A Merkle tree is a binary tree that has two main types of nodes:
/// - leafs that contain a value (usually block) and it's hash;
/// - nodes that contain a hash of a left and right nodes hashes concatenation;
///                         h0123
///                        /     \
///                      h01     h23
///                     /   \   /   \
///                    h0   h1 h2    h3
#[derive(Debug)]
pub struct MerkleTree<T> {
    /// The root of the inner binary tree
    root: Tree<T>,

    /// The height of the tree
    height: usize,

    /// The number of leaf nodes in the tree
    count: usize,

    /// Hashmap value hash -> leaf indices
    map: std::collections::HashMap<u64, usize>,
}

/// Proof generated from the Merkel tree
/// Proof contains the hash path and the target value
#[derive(Debug, PartialEq)]
pub struct Proof<T> {
    /// The path to the value
    pub path: Vec<u64>,

    /// The target value
    pub value: T,
}

impl<T: Clone> MerkleTree<T> {
    /// Constructs a Merkle Tree from a vector of data blocks.
    /// Returns `None` if `values` is empty.
    pub fn build_tree(values: Vec<T>) -> Self
    where
        T: Hash,
    {
        if values.is_empty() {
            return MerkleTree {
                root: Tree::empty(),
                height: 0,
                count: 0,
                map: std::collections::HashMap::new(),
            };
        }

        let count = values.len();
        let mut cur = Vec::with_capacity(count);
        let mut next = Vec::with_capacity(count / 2 + 1);

        let mut index = 0usize;
        let mut map: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

        for value in values {
            let leaf = Tree::new_leaf(value, index);
            map.insert(leaf.hash(), index);
            cur.push(leaf);
            index = index + 1;
        }

        let mut height = 0;

        while cur.len() > 1 {
            while cur.len() > 0 {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let mut hasher = DefaultHasher::new();
                    (left.hash(), right.hash()).hash(&mut hasher);
                    let combined_hash = hasher.finish();

                    let left_index = left.get_left_index().unwrap();
                    let right_index = right.get_right_index().unwrap();
                    let node = Tree::Node {
                        hash: combined_hash,
                        left: Box::new(left),
                        right: Box::new(right),
                        left_index: left_index,
                        right_index: right_index,
                    };

                    next.push(node);
                }
            }

            height += 1;
            std::mem::swap(&mut cur, &mut next);
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);
        debug_assert!(root.get_left_index().unwrap() == 0usize);
        debug_assert!(root.get_right_index().unwrap() == count - 1);

        MerkleTree {
            root,
            height,
            count,
            map,
        }
    }

    /// Returns the root hash of Merkle tree
    pub fn root_hash(&self) -> u64 {
        self.root.hash().clone()
    }

    /// Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of leaves in the Merkle tree
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns whether the Merkle tree is empty or not
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Generate an inclusion proof for the given value.
    /// Returns `None` if the given value is not found in the tree.
    pub fn gen_proof(&self, value: T) -> Option<Proof<T>>
    where
        T: Hash + PartialEq,
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let value_hash = hasher.finish();

        if !self.map.contains_key(&value_hash) {
            return None;
        }

        let leaf_index = self.map.get(&value_hash).unwrap();
        let proof = self.gen_nth_proof(*leaf_index);
        if !proof.is_some() {
            return None;
        }
        if value != proof.as_ref().unwrap().value {
            return None;
        }
        return proof;
    }

    /// Generate an inclusion proof for the `n`-th leaf value.
    pub fn gen_nth_proof(&self, n: usize) -> Option<Proof<T>> {
        let mut path: Vec<u64> = Vec::new();
        return self.get_nth_proof_impl(&mut path, &self.root, n);
    }

    /// Generate hash path starting from the root hash and ending with leaf hash.
    fn get_nth_proof_impl(
        &self,
        path: &mut Vec<u64>,
        tree: &Tree<T>,
        leaf_index: usize,
    ) -> Option<Proof<T>> {
        match tree {
            Tree::Empty { .. } => return None,
            Tree::Leaf { value, .. } => {
                debug_assert!(tree.get_left_index() == tree.get_right_index());

                if tree.get_left_index().unwrap() != leaf_index {
                    return None;
                }

                path.push(tree.hash());
                let proof: Proof<T> = Proof {
                    path: path.clone(),
                    value: value.clone(),
                };
                return Some(proof);
            }
            Tree::Node {
                ref right, left, ..
            } => {
                if !tree.get_left_index().is_some() {
                    return None;
                }
                if !tree.get_right_index().is_some() {
                    return None;
                }
                let left_index = tree.get_left_index().unwrap();
                let right_index = tree.get_right_index().unwrap();
                if leaf_index < left_index {
                    return None;
                }
                if leaf_index > right_index {
                    return None;
                }

                path.push(tree.hash());
                let proof = self.get_nth_proof_impl(path, &*right, leaf_index);
                if proof.is_some() {
                    return proof;
                }

                let proof = self.get_nth_proof_impl(path, &*left, leaf_index);
                return proof;
            }
        }
    }

    /// Returns true if proof is valid, false otherwise.
    pub fn validate_proof(&self, proof_to_verify: &Proof<T>) -> bool
    where
        T: Hash + PartialEq,
    {
        // TODO: optimize by traversing through tree instead of comparison
        let generated_proof = self.gen_proof(proof_to_verify.value.clone());
        if !generated_proof.is_some() {
            return false;
        }
        return generated_proof.unwrap() == *proof_to_verify;
    }
}
