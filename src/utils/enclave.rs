use crate::primitives::MrEnclave;
use basex_rs::{BaseX, ALPHABET_BITCOIN};

pub fn mrenclave_to_bs58(mrenclave: &MrEnclave) -> String {
    BaseX::with_alphabet(ALPHABET_BITCOIN).to_bs58(mrenclave)
}

pub fn mrenclave_from_bs58(mrenclave: String) -> Result<MrEnclave, String> {
    match BaseX::with_alphabet(ALPHABET_BITCOIN).from_bs58(&mrenclave) {
        Some(m) => {
            let mut bytes = [0u8; 32];
            bytes[..32].clone_from_slice(&m);

            Ok(bytes)
        }
        None => Err("Decode base58 error".into()),
    }
}

pub fn mock_a_shard() -> MrEnclave {
    let shard: MrEnclave = rand::random();
    shard
}
