#![no_std]

mod merkle_tree;
mod poseidon_hasher;

type U256 = ruint::aliases::U256;

use merkle_tree::*;
use poseidon_hasher::*;

extern crate alloc;
use alloc::vec::Vec;

use runtime::coprocessors::poseidon_gl;

#[no_mangle]
fn main() {
    //zero_test();
    empty_proof();
    //zero_and_nonexisting_is_same();
}

fn zero_test() {
    let mut p = PoseidonHasher::default();
    p.write_h256(&U256::ZERO);
    p.write_h256(&U256::ZERO);
    let h = p.finish();
    let h_limbs = h.as_limbs();
    assert_eq!(h_limbs[0], 4330397376401421145);
    assert_eq!(h_limbs[1], 14124799381142128323);
    assert_eq!(h_limbs[2], 8742572140681234676);
    assert_eq!(h_limbs[3], 14345658006221440202);
}

const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);

fn empty_proof() {
    let tree = MerkleTree::<PoseidonHasher, U256>::default();
    let proof = tree.proof(&ONE);
    //let root_hash = tree.root_hash();
    assert_eq!(proof.len(), N_LEAVES);
    /*
    assert!(MerkleTree::<PoseidonHasher, U256>::verify_proof(
        &root_hash,
        &ONE,
        &U256::ZERO,
        &proof
    ));
    */
}

fn zero_and_nonexisting_is_same() {
    /*
        let mut tree = MerkleTree::<PoseidonHasher, U256>::default();
        let empty_root_hash = tree.root_hash();
        tree.update(&1, 0);
    */
        //assert_eq!(tree.root_hash(), empty_root_hash);
        /*
        tree.update(&0, 0);
        assert_eq!(tree.root_hash(), empty_root_hash);
        tree.update(&23, 0);
        assert_eq!(tree.root_hash(), empty_root_hash);
        */

        /*
        tree.update(&0, 1);
        let something_at_zero = tree.root_hash();
        assert!(something_at_zero != empty_root_hash);
        tree.update(&1, 7);
        let something_at_one_and_zero = tree.root_hash();
        assert!(something_at_one_and_zero != empty_root_hash);
        assert!(something_at_one_and_zero != something_at_zero);

        tree.delete(&0);
        let something_at_one = tree.root_hash();
        assert!(something_at_one != something_at_one_and_zero);
        assert!(something_at_one != something_at_zero);
        assert!(something_at_one != empty_root_hash);

        tree.delete(&1);
        assert_eq!(tree.root_hash(), empty_root_hash);
        */
    }
