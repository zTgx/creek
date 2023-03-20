use crate::primitives::{
    Address20, Address32, AesOutput, Credential, USER_SHIELDING_KEY_NONCE_LEN,
};
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, Key, KeyInit,
};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_json;
use sha2::Sha256;

pub fn encrypt_with_tee_shielding_pubkey(
    tee_shielding_pubkey: RsaPublicKey,
    msg: &[u8],
) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), msg)
        .expect("failed to encrypt")
}

pub fn decrypt_vc_with_user_shielding_key(
    encrypted_vc: AesOutput,
    user_shielding_key: &[u8],
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
    encrypted_challenge_code: AesOutput,
    user_shielding_key: &[u8],
) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_challenge_code.ciphertext;
    let nonce = encrypted_challenge_code.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decrypt ChallengeCode Error: {:?}", e))
}

pub fn hex_account_to_address32(hex_account: &str) -> Result<Address32, &'static str> {
    if !hex_account.starts_with("0x") && hex_account.len() != 62 {
        return Err("Incorrect hex account format!");
    }

    let decoded_account = hex::decode(&hex_account[2..]).unwrap();
    let mut bytes = [0u8; 32];
    bytes[..32].clone_from_slice(&decoded_account);

    Ok(Address32::from(bytes))
}

pub fn hex_account_to_address20(hex_account: &str) -> Result<Address20, &'static str> {
    if !hex_account.starts_with("0x") && hex_account.len() != 42 {
        return Err("Incorrect hex account format!");
    }

    let decoded_account = hex::decode(&hex_account[2..]).unwrap();
    let mut bytes = [0u8; 20];
    bytes[..20].clone_from_slice(&decoded_account);

    Ok(Address20::from(bytes))
}

pub fn print_passed() {
    println!(" 🎉 All testcases passed!");
}

pub fn print_failed(reason: String) {
    println!(" ❌ Testcase failed, reason: {}", reason);
}
