use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, Key, KeyInit,
};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_json;
use sha2::Sha256;
use crate::{
    primitives::{AesOutput, Credential, USER_SHIELDING_KEY_NONCE_LEN}
};

pub fn encrypt_with_tee_shielding_pubkey(
    tee_shielding_pubkey: RsaPublicKey,
    msg: &[u8],
) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}

pub fn decrypt_vc_with_user_shielding_key(encrypted_vc: AesOutput, user_shielding_key: &[u8]) -> Result<Credential, String> {
    let ciphertext = encrypted_vc.ciphertext;
    let nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN] = encrypted_vc.nonce;

    let key = Key::<Aes256Gcm>::from_slice(&user_shielding_key);
    let nonce = GenericArray::from_slice(&nonce);
    let cipher = Aes256Gcm::new(key);
    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => {
            serde_json::from_slice(&plaintext).map_err(
                |e| format!("Deserialize VC error: {:?}", e)
            )
        }
        Err(e) => {
            Err(format!("Deserialize VC error: {:?}", e))
        }
    }
}

pub fn decrypt_challage_code_with_user_shielding_key(encrypted_challenge_code: AesOutput, user_shielding_key: &[u8]) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(&user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_challenge_code.ciphertext;
    let nonce = encrypted_challenge_code.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    cipher.decrypt(nonce, ciphertext.as_ref()).map_err(
        |e| format!("Decrypt ChallengeCode Error: {:?}", e)
    )
}

pub fn print_passed() {
    println!(" ✅ >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 🎉");
}

pub fn print_failed() {
    println!(" ❌ >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 🚩");
}
