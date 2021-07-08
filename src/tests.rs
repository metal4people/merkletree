#![cfg(test)]

use crate::merkletree::MerkleTree;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_value<T>(value: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    return hasher.finish();
}

#[test]
fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];

    let hashes = vec![
        hash_value(&values[0]),
        hash_value(&values[1]),
        hash_value(&values[2]),
        hash_value(&values[3]),
    ];

    let count = values.len();
    let tree = MerkleTree::build_tree(values);

    let h01 = hash_value((hashes[0], hashes[1]));
    let h23 = hash_value((hashes[2], hashes[3]));

    let root_hash = hash_value((h01, h23));

    assert_eq!(tree.count(), count);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash(), root_hash);
}

#[test]
fn test_build_tree_empty() {
    let values: Vec<Vec<u8>> = vec![];
    let tree = MerkleTree::build_tree(values);

    let mut hasher = DefaultHasher::new();
    "".hash(&mut hasher);
    let empty_hash = hasher.finish();

    let root_hash = tree.root_hash().clone();

    assert_eq!(root_hash, empty_hash);
}

#[test]
fn test_build_tree1() {
    let values = vec!["hello, world".to_string()];
    let root_hash = hash_value(&values[0]);
    let tree = MerkleTree::build_tree(values);

    assert_eq!(tree.count(), 1);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash(), root_hash);
}

#[test]
fn test_build_tree3() {
    let values = vec![vec![1], vec![2], vec![3]];
    let tree = MerkleTree::build_tree(values);

    let hashes = vec![
        hash_value(&vec![1]),
        hash_value(&vec![2]),
        hash_value(&vec![3]),
    ];

    let h01 = hash_value((&hashes[0], &hashes[1]));
    let h2 = &hashes[2];
    let root_hash = hash_value((&h01, h2));

    assert_eq!(tree.count(), 3);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash(), root_hash);
}

#[test]
fn test_build_tree9() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();

    let hashes = values.iter().map(|v| hash_value(v)).collect::<Vec<_>>();

    let tree = MerkleTree::build_tree(values);

    let h01 = hash_value((&hashes[0], &hashes[1]));
    let h23 = hash_value((&hashes[2], &hashes[3]));
    let h45 = hash_value((&hashes[4], &hashes[5]));
    let h67 = hash_value((&hashes[6], &hashes[7]));
    let h8 = &hashes[8];
    let h0123 = hash_value((&h01, &h23));
    let h4567 = hash_value((&h45, &h67));
    let h1to7 = hash_value((&h0123, &h4567));

    let root_hash = hash_value((&h1to7, h8));

    assert_eq!(tree.count(), 9);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.root_hash(), root_hash);
}

#[test]
fn test_valid_proof() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::build_tree(values.clone());

    for value in values {
        let proof = tree.gen_proof(value);
        assert!(proof.is_some());
        let is_valid = tree.validate_proof(&proof.unwrap());
        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let values = vec!["Hello", "my", "name", "is", "Rusty"];
    let tree = MerkleTree::build_tree(values);

    let value = "Rusty";

    let proof = tree.gen_proof(&value);
    assert!(proof.is_some());
    let is_valid = tree.validate_proof(&proof.unwrap());

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let values1 = vec![vec![1], vec![2], vec![3], vec![4]];
    let tree1 = MerkleTree::build_tree(values1.clone());

    let values2 = vec![vec![4], vec![5], vec![6], vec![7]];
    let tree2 = MerkleTree::build_tree(values2);

    for value in values1 {
        let proof = tree1.gen_proof(value);
        assert!(proof.is_some());

        let is_valid = tree2.validate_proof(&proof.unwrap());
        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_nth_proof() {
    // Calculation depends on the total count. Try a few numbers: odd, even, powers of two...
    for &count in &[1, 2, 3, 10, 15, 16, 17, 22] {
        let values = (1..=count).map(|x| vec![x as u8]).collect::<Vec<_>>();
        let tree = MerkleTree::build_tree(values.clone());

        for i in 0..count {
            let proof = tree.gen_nth_proof(i);
            assert!(proof.is_some());
            assert_eq!(vec![i as u8 + 1], proof.as_ref().unwrap().value);
            assert!(tree.validate_proof(&proof.unwrap()));
        }

        assert!(tree.gen_nth_proof(count).is_none());
        assert!(tree.gen_nth_proof(count + 1000).is_none());
    }
}
