//! *merkle* implements a Merkle Tree in Rust.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod merkletree;
pub use crate::merkletree::MerkleTree;

mod tree;
pub use crate::tree::Tree;

#[cfg(test)]
mod tests;
