use crate::{get_tee_shielding_pubkey};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use sha2::Sha256;

pub fn encrypt_with_tee_shielding_pubkey(msg: &[u8]) -> Vec<u8> {
    let tee_shielding_pubkey: RsaPublicKey = get_tee_shielding_pubkey();
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}

