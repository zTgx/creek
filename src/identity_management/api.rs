use super::{build_create_identity_extrinsic, build_set_user_shielding_key_extrinsic};
use crate::{
    primitives::{Address32, Identity, MrEnclave},
    send_extrinsic,
    utils::encrypt_with_tee_shielding_pubkey,
};
use codec::Encode;

pub fn set_user_shielding_key(shard: MrEnclave, aes_key: Vec<u8>) {
    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);
    let xt = build_set_user_shielding_key_extrinsic(shard, encrpted_shielding_key);
    send_extrinsic(xt.hex_encode());
}

pub fn create_identity(
    shard: MrEnclave,
    address: Address32,
    identity: Identity,
    ciphertext_metadata: Option<Vec<u8>>,
) {
    let identity_encoded = identity.encode();
    let ciphertext = encrypt_with_tee_shielding_pubkey(&identity_encoded);
    // let ciphertext_metadata: Option<Vec<u8>> = None;

    let xt = build_create_identity_extrinsic(shard, address, ciphertext, ciphertext_metadata);
    send_extrinsic(xt.hex_encode());
}
