# How this was made

The purpose of this document is to describe the flow of thoughts during solving the task.

## Attitude
I've heard about the Merkle tree kind of week before starting on this task. I thought that it would be very nice to learn about it more.
This task to me is very interesting cause it's a combo of Merkle tree plus rust, the language about which I'd like to learn more.

## Resources
Didn't have much time during these days cause of the regular working hours, mainly developed it early morning and late evening.

## Analysis
The first thing what I did, I read about the Merkle tree and got the idea: if the blockchain was represented as a linked list to verify a transaction, it would be needed to traverse the list from the very beginning that would be O(n) time. Merkle trees solve the problem by doing it for log(n) time.

After understanding its purpose I've started to analyze existing solutions on github, one that looked nice to me was https://github.com/SpinResearch/merkle.rs.

While reading about merkle trees, I've found one more interesting aspect about working with blockchain related code - it should not only be reliable, run fast, consume as less memory as possible, but also be secure.

From security perspective, I've found that guys from SpinResearch rely only on hashes and don't compare the actual value of data (block) that's in the leafs.

In addition, it seemed that code that was used for building the proof and proof validation was really complex and hard to understand and also can be faster.

## Implementation

I decided to re-write SpinResearch implementation, in order to speed up proof lookup by introducing tree segmentation: each node contains the right and left indices of the right most and left most leaf indices of the current subtree.

While generating the proof it would be simple binary search along the tree, the same could be done when the proof is validated.

I've also simplified interface by removing the hash algorithm and use the default one. 

## Testing and benchmarking

Since I wanted to compare the current implementation with initial one, the tests and benchmarking I left as it is, so benchmarks between two projects can be compared.

Below is the comparison of the results from cargo bench:

| Benchmark test name                      | Original impl, time| Re-written impl, time |
|------------------------------------------|---------------|-----------------------|
| MerkleTree::build_tree - small           | 2.4838 us     | 457 ns                |
| MerkleTree::gen_proof - small            | 1.5685 us     | 349 ns                |
| MerkleTree::validate_proof - small       | 3.6923 us     | 352 ns                |
| MerkleTree::build_tree - big             | 219.33 us     | 56.775 us             |
| MerkleTree::gen_proof - big              | 315.60 us     | 51.514 us             |
| MerkleTree::validate_proof - big         | 530.70 us     | 53.869 us             |

For more details there is benchmark files for both implementations inside
benches directory.

## Thoughts about possible improvements 
- Theoretically it is possible to store nodes of the tree contiguously in the array, it would be nice to experiment with it;
- Would be interesting to support updating the tree along with making it thread-safe; 
- Currently 64-bit hashes are used, but it may not be sufficient;
- Proof validation can be optimized without using proof generation;