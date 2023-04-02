use crate::primitives::VCIndex;

pub fn create_a_random_vc_index() -> VCIndex {
    let index: [u8; 32] = rand::random();
    VCIndex::from(index)
}
