use crate::{get_tee_shielding_pubkey, primitives::Credential, LIT_Aes256G_KEY};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use sha2::Sha256;
use aes_gcm::{Key, Aes256Gcm, KeyInit};

pub fn encrypt_with_tee_shielding_pubkey(msg: &[u8]) -> Vec<u8> {
    let tee_shielding_pubkey: RsaPublicKey = get_tee_shielding_pubkey();
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}

// pub fn decryptWithAES(msg: &[u8]) -> () {
//     let aes_key = LIT_Aes256G_KEY.to_vec();
//     let key = Key::<Aes256Gcm>::from_slice(aes_key);
//     let cipher = Aes256Gcm::new(&key);
//     let plaintext = cipher.decrypt(ciphertext.as_ref())?;

// }