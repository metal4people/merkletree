#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate merkletree;
extern crate rand;

use rand::RngCore;
use merkletree::MerkleTree;

fn bench_small_str_tree(c: &mut Criterion) {
    c.bench_function("MerkleTree::build_tree - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        b.iter(|| MerkleTree::build_tree(black_box(values.clone())))
    });
}

fn bench_small_str_proof_gen(c: &mut Criterion) {
    c.bench_function("MerkleTree::gen_proof - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::build_tree(values.clone());

        b.iter(|| {
            for value in &values {
                tree.gen_proof(black_box(value));
            }
        })
    });
}

fn bench_small_str_proof_check(c: &mut Criterion) {
    c.bench_function("MerkleTree::validate_proof - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::build_tree(values.clone());
        let proofs = values
            .iter()
            .map(|v| tree.gen_proof(v).unwrap())
            .collect::<Vec<_>>();

        b.iter(|| {
            for proof in &proofs {
                tree.validate_proof(proof);
            }
        })
    });
}

fn bench_big_rnd_tree(c: &mut Criterion) {
    c.bench_function("MerkleTree::build_tree - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        b.iter(|| MerkleTree::build_tree(black_box(values.clone())))
    });
}

fn bench_big_rnd_proof_gen(c: &mut Criterion) {
    c.bench_function("MerkleTree::gen_proof - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        let tree = MerkleTree::build_tree(values.clone());

        b.iter(|| {
            for value in &values {
                tree.gen_proof(black_box(value.clone()));
            }
        })
    });
}

fn bench_big_rnd_proof_check(c: &mut Criterion) {
    c.bench_function("MerkleTree::validate_proof - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        let tree = MerkleTree::build_tree(values.clone());
        let proofs = values
            .into_iter()
            .map(|v| tree.gen_proof(v).unwrap())
            .collect::<Vec<_>>();

        b.iter(|| {
            for proof in &proofs {
                tree.validate_proof(proof);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_small_str_tree,
    bench_small_str_proof_gen,
    bench_small_str_proof_check,
    bench_big_rnd_tree,
    bench_big_rnd_proof_gen,
    bench_big_rnd_proof_check,
);

criterion_main!(benches);