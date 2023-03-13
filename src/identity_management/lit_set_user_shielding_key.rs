use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use sha2::Sha256;
use sp_core::H256;
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus};
use crate::{get_shard, get_tee_shielding_pubkey, API};

pub fn tc00_set_user_shielding_key() {
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    println!("  [SetUserShieldingKey]-TC00 aes_key: {:?}", aes_key);

    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);

    let shard = get_shard();
    println!("  [SetUserShieldingKey]-TC00 shard: {:?}", shard);

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key.to_vec()
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

fn encrypt_with_tee_shielding_pubkey(msg: &[u8]) -> Vec<u8> {
    let tee_shielding_pubkey: RsaPublicKey = get_tee_shielding_pubkey();
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}
