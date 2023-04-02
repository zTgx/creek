use sp_core::H256;

pub fn create_a_random_vc_index() -> H256 {
    let index: [u8; 32] = rand::random();
    H256::from(index)
}
