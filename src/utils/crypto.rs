use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, Key, KeyInit};
use codec::Decode;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_json;
use sha2::Sha256;

use crate::primitives::crypto::AesOutput;
use crate::primitives::identity::{Identity, IdentityContext};
use crate::primitives::vc::Credential;
use crate::primitives::{ChallengeCode, CHALLENGE_CODE_SIZE, USER_SHIELDING_KEY_NONCE_LEN};

pub fn generate_user_shielding_key() -> Vec<u8> {
    let user_shieldng_key = Aes256Gcm::generate_key(&mut OsRng);
    user_shieldng_key.to_vec()
}

pub fn generate_incorrect_user_shielding_key() -> Vec<u8> {
    [0, 1].to_vec()
}

pub fn encrypt_with_tee_shielding_pubkey(
    tee_shielding_pubkey: &RsaPublicKey,
    msg: &[u8],
) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}

pub fn encrypt_with_user_shielding_key(
    user_shielding_key: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, String> {
    let nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN] = rand::random();

    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let nonce = GenericArray::from_slice(&nonce);
    let cipher = Aes256Gcm::new(key);
    match cipher.encrypt(nonce, plaintext) {
        Ok(encrypted) => Ok(encrypted),
        Err(e) => Err(format!("encrypt error: {:?}", e)),
    }
}

pub fn decrypt_vc_with_user_shielding_key(
    user_shielding_key: &[u8],
    encrypted_vc: AesOutput,
) -> Result<Credential, String> {
    let ciphertext = encrypted_vc.ciphertext;
    let nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN] = encrypted_vc.nonce;

    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let nonce = GenericArray::from_slice(&nonce);
    let cipher = Aes256Gcm::new(key);
    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => {
            serde_json::from_slice(&plaintext).map_err(|e| format!("Deserialize VC error: {:?}", e))
        }
        Err(e) => Err(format!("Deserialize VC error: {:?}", e)),
    }
}

pub fn decrypt_challage_code_with_user_shielding_key(
    user_shielding_key: &[u8],
    encrypted_challenge_code: AesOutput,
) -> Result<ChallengeCode, String> {
    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_challenge_code.ciphertext;
    let nonce = encrypted_challenge_code.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    let code = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decrypt ChallengeCode Error: {:?}", e))?;

    let mut challenge_code: ChallengeCode = [0u8; CHALLENGE_CODE_SIZE];
    challenge_code[..CHALLENGE_CODE_SIZE].clone_from_slice(&code);

    Ok(challenge_code)
}

pub fn decrypt_identity_with_user_shielding_key(
    user_shielding_key: &[u8],
    encrypted_identity: AesOutput,
) -> Result<Identity, String> {
    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_identity.ciphertext;
    let nonce = encrypted_identity.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => Identity::decode(&mut plaintext.as_slice())
            .map_err(|e| format!("Decode identity error: {}", e)),
        Err(e) => Err(format!("Decode identity error: {}", e)),
    }
}

pub fn decrypt_id_graph_with_user_shielding_key(
    user_shielding_key: &[u8],
    encrypted_id_graph: AesOutput,
) -> Result<Vec<(Identity, IdentityContext)>, String> {
    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_id_graph.ciphertext;
    let nonce = encrypted_id_graph.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(plaintext) => Vec::<(Identity, IdentityContext)>::decode(&mut plaintext.as_slice())
            .map_err(|e| format!("Decode identity error: {}", e)),
        Err(e) => Err(format!("Decode identity error: {}", e)),
    }
}
