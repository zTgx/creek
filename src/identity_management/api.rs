use crate::{
    get_shard,
    primitives::{Address32, Identity, MrEnclave},
    send_extrinsic,
    utils::encrypt_with_tee_shielding_pubkey,
};
use codec::Encode;

use super::{build_create_identity_extrinsic, build_set_user_shielding_key_extrinsic};

pub fn set_user_shielding_key(shard: MrEnclave, aes_key: Vec<u8>) {
    // let aes_key = USER_AES256G_KEY.to_vec();

    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);

    let xt = build_set_user_shielding_key_extrinsic(shard, encrpted_shielding_key);
    send_extrinsic(xt.hex_encode());
}

pub fn create_identity(
    address: Address32,
    identity: Identity,
    ciphertext_metadata: Option<Vec<u8>>,
) {
    // let add = hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    // let mut y = [0u8; 32];
    // y[..32].clone_from_slice(&add);

    // let address = Address32::from(y);

    // let network = SubstrateNetwork::Litentry;
    // let identity = Identity::Substrate { network, address };

    let shard = get_shard();
    let msg = identity.encode();
    let ciphertext = encrypt_with_tee_shielding_pubkey(&msg);
    // let ciphertext_metadata: Option<Vec<u8>> = None;

    let xt = build_create_identity_extrinsic(shard, address, ciphertext, ciphertext_metadata);
    send_extrinsic(xt.hex_encode());
}
