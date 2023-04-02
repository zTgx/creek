use crate::primitives::MrEnclave;
use basex_rs::{BaseX, Decode as Base58Decode, Encode as Base58Encode, BITCOIN};

pub fn mrenclave_to_bs58(mrenclave: &MrEnclave) -> String {
    BaseX::new(BITCOIN).encode(mrenclave)
}

pub fn mrenclave_from_bs58(mrenclave: String) -> Result<MrEnclave, String> {
    match BaseX::new(BITCOIN).decode(mrenclave) {
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
