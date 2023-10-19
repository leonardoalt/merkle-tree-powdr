extern crate alloc;
use alloc::vec::Vec;

use runtime::coprocessors::poseidon_gl;

pub trait Hasher {
    fn write_h256(&mut self, w: &U256);
    fn finish(&mut self) -> U256;
}

use ruint::aliases::U256;

const GOLDILOCKS: u64 = 0xffffffff00000001;

#[derive(Default, Clone)]
// Each u64 here must be < Goldilocks.
pub struct PoseidonHasher(Vec<u64>);

impl Hasher for PoseidonHasher {
    fn write_h256(&mut self, w: &U256) {
        for limb in w.as_limbs() {
            if limb < &GOLDILOCKS {
                self.0.push(*limb);
            } else {
                let lower = *limb as u32;
                let upper = (limb >> 32) as u32;
                self.0.push(lower as u64);
                self.0.push(upper as u64);
            }
        }
    }

    fn finish(&mut self) -> U256 {
        let (first_chunk, rest) = self.0.split_at(4);

        let initial_acc = U256::from_limbs(first_chunk.try_into().unwrap());

        let acc = rest.chunks(4).fold(initial_acc, |acc, chunk| {
            let mut i = [0; 12];
            i[..4].copy_from_slice(acc.as_limbs());
            i[4..4 + chunk.len()].copy_from_slice(chunk);
            U256::from_limbs(poseidon_gl(i))
        });

        acc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hasher() {
        let mut hasher = PoseidonHasher::default();
        hasher.write_h256(&0.into());
        hasher.write_h256(&1.into());
        assert_eq!(
            hasher.finish(),
            0
        );
    }
}
