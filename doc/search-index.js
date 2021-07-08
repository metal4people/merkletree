var searchIndex = JSON.parse('{\
"lib":{"doc":"<em>merkle</em> implements a Merkle Tree in Rust.","t":[13,13,3,13,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12],"n":["Empty","Leaf","MerkleTree","Node","Tree","borrow","borrow","borrow_mut","borrow_mut","build_tree","clone","clone_into","count","empty","fmt","fmt","from","from","gen_nth_proof","gen_proof","get_left_index","get_right_index","hash","height","into","into","is_empty","new_leaf","root_hash","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","validate_proof","hash","hash","hash","index","left","left_index","right","right_index","value"],"q":["lib","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","lib::Tree","","","","","","","",""],"d":["Empty node","Leaf node","A Merkle tree is a binary tree that has two main types of …","Node that has both right and left nodes","Binary tree that’s used for Merkle tree implementation","","","","","Constructs a Merkle Tree from a vector of data blocks. …","","","Returns the number of leaves in the Merkle tree","Create an empty tree","","","","","Generate an inclusion proof for the <code>n</code>-th leaf value.","Generate an inclusion proof for the given value. Returns …","Returns a left index of the covered tree segment …","Returns a right index of the covered tree segment …","Returns a hash from the tree","Returns the height of Merkle tree","","","Returns whether the Merkle tree is empty or not","Create new leaf with predefined T value and index","Returns the root hash of Merkle tree","","","","","","","","Returns true if proof is valid, false otherwise.","Hash for the empty value","Value hash","Hash of (right sub-tree hash + left sub-tree hash)","Index in the array: the left most leaf has 0 index","Left sub-tree","Index of the left most leaf covered by the current tree","Right sub-tree","Index of the right most leaf covered by the current tree","Hashed value"],"i":[1,1,0,1,0,2,1,2,1,2,1,1,2,1,2,1,2,1,2,2,1,1,1,2,2,1,2,1,2,1,2,1,2,1,2,1,2,3,4,5,4,5,5,5,5,4],"f":[null,null,null,null,null,[[]],[[]],[[]],[[]],[[["vec",3]]],[[],["tree",4]],[[]],[[],["usize",15]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["usize",15]],[["proof",3],["option",4]]],[[],[["proof",3],["option",4]]],[[],[["option",4],["usize",15]]],[[],[["option",4],["usize",15]]],[[],["u64",15]],[[],["usize",15]],[[]],[[]],[[],["bool",15]],[[["usize",15]],["tree",4]],[[],["u64",15]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[["proof",3]],["bool",15]],null,null,null,null,null,null,null,null,null],"p":[[4,"Tree"],[3,"MerkleTree"],[13,"Empty"],[13,"Leaf"],[13,"Node"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};