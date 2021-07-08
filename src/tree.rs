use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Binary tree that's used for Merkle tree implementation
#[derive(Clone)]
pub enum Tree<T> {
    /// Empty node
    Empty {
        /// Hash for the empty value
        hash: u64,
    },

    /// Leaf node
    Leaf {
        /// Value hash
        hash: u64,
        /// Hashed value
        value: T,
        /// Index in the array: the left most leaf has 0 index
        index: usize,
    },

    /// Node that has both right and left nodes
    Node {
        /// Hash of (right sub-tree hash + left sub-tree hash)
        hash: u64,
        /// Left sub-tree
        left: Box<Tree<T>>,
        /// Right sub-tree
        right: Box<Tree<T>>,
        /// Index of the left most leaf covered by the current tree
        left_index: usize,
        /// Index of the right most leaf covered by the current tree
        right_index: usize,
    },
}

impl<T> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Tree::Empty { ref hash } => {
                return f.debug_struct("Tree::Empty").field("hash", hash).finish()
            }
            Tree::Leaf { ref hash, .. } => {
                return f.debug_struct("Tree::Leaf").field("hash", hash).finish()
            }
            Tree::Node { ref hash, .. } => {
                return f.debug_struct("Tree::Node").field("hash", hash).finish()
            }
        }
    }
}

impl<T> Tree<T> {
    /// Create an empty tree
    pub fn empty() -> Self {
        let mut hasher = DefaultHasher::new();
        "".hash(&mut hasher);
        let hash = hasher.finish();

        Tree::Empty { hash }
    }

    /// Create new leaf with predefined T value and index
    pub fn new_leaf(value: T, index: usize) -> Tree<T>
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        Tree::Leaf {
            hash: hash.clone(),
            value: value,
            index: index,
        }
    }

    /// Returns a hash from the tree
    pub fn hash(&self) -> u64 {
        match *self {
            Tree::Empty { ref hash } => hash.clone(),
            Tree::Leaf { ref hash, .. } => hash.clone(),
            Tree::Node { ref hash, .. } => hash.clone(),
        }
    }

    /// Returns a left index of the covered tree segment (inclusively)
    pub fn get_left_index(&self) -> Option<usize> {
        match *self {
            Tree::Empty { .. } => None,
            Tree::Leaf { ref index, .. } => Some(index.clone()),
            Tree::Node { ref left_index, .. } => Some(left_index.clone()),
        }
    }

    /// Returns a right index of the covered tree segment (inclusively)
    pub fn get_right_index(&self) -> Option<usize> {
        match *self {
            Tree::Empty { .. } => None,
            Tree::Leaf { ref index, .. } => Some(index.clone()),
            Tree::Node {
                ref right_index, ..
            } => Some(right_index.clone()),
        }
    }
}
