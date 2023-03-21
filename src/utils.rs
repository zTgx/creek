use crate::{
    primitives::{
        Address20, Address32, AesOutput, ChallengeCode, Credential, Identity,
        IdentityMultiSignature, ValidationData, ValidationString, Web3CommonValidationData,
        Web3ValidationData, CHALLENGE_CODE_SIZE, USER_SHIELDING_KEY_NONCE_LEN,
    },
    ACCOUNT_SEED_CHARSET,
};
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    Aes256Gcm, Key, KeyInit,
};
use codec::{Decode, Encode};
use rand::{Rng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_json;
use sha2::Sha256;
use sp_core::sr25519::Pair as SubstratePair;
use sp_core::{blake2_256, sr25519, Pair}; // TODO: maybe use more generic struct

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
) -> Result<ChallengeCode, String> {
    let key = Key::<Aes256Gcm>::from_slice(user_shielding_key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = encrypted_challenge_code.ciphertext;
    let nonce = encrypted_challenge_code.nonce;
    let nonce = GenericArray::from_slice(&nonce);
    let code = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decrypt ChallengeCode Error: {:?}", e));

    let mut challenge_code: ChallengeCode = [0u8; CHALLENGE_CODE_SIZE];
    challenge_code[..CHALLENGE_CODE_SIZE].clone_from_slice(&code.unwrap());

    Ok(challenge_code)
}

pub fn decrypt_identity_with_user_shielding_key(
    encrypted_identity: AesOutput,
    user_shielding_key: &[u8],
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

pub fn get_a_random_u32() -> u32 {
    let mut os_rng = OsRng;
    os_rng.next_u32()
}

/// Interprets the string `s` in order to generate a key Pair. Returns both the pair and an
/// optional seed, in the case that the pair can be expressed as a direct derivation from a seed
/// (some cases, such as Sr25519 derivations with path components, cannot).
///
/// This takes a helper function to do the key generation from a phrase, password and
/// junction iterator.
///
/// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will be interpreted
/// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
/// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words, then the key will
/// be derived from it. In this case:
///   - the phrase may be followed by one or more items delimited by `/` characters.
///   - the path may be followed by `///`, in which case everything after the `///` is treated
/// as a password.
/// - If `s` begins with a `/` character it is prefixed with the Substrate public `DEV_PHRASE`
///   and
/// interpreted as above.
///
/// In this case they are interpreted as HDKD junctions; purely numeric items are interpreted as
/// integers, non-numeric items as strings. Junctions prefixed with `/` are interpreted as soft
/// junctions, and with `//` as hard junctions.
///
/// There is no correspondence mapping between SURI strings and the keys they represent.
/// Two different non-identical strings can actually lead to the same secret being derived.
/// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
/// Similarly an empty password (ending the SURI with `///`) is perfectly valid and will
/// generally be equivalent to no password at all.
///
/// `None` is returned if no matches are found.
/// See `from_string_with_seed` for more details in substrate.
pub fn get_random_account_seed(len: usize) -> String {
    let mut rng = rand::thread_rng();

    let account_str: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..ACCOUNT_SEED_CHARSET.len());
            ACCOUNT_SEED_CHARSET[idx] as char
        })
        .collect();

    account_str
}

pub fn create_n_random_sr25519_address(num: u32) -> Vec<sr25519::Pair> {
    let mut addresses = vec![];
    let mut index = 0;
    while index < num {
        let mut account_seed = get_random_account_seed(3);
        account_seed.insert_str(0, "//");
        let account_pair = sr25519::Pair::from_string(&account_seed, None).unwrap();
        // let address: Address32 = account_pair.public().0.into();

        addresses.push(account_pair);

        index += 1;
    }

    addresses
}

fn get_expected_raw_message(who: &Address32, identity: &Identity, code: &ChallengeCode) -> Vec<u8> {
    let mut payload = code.encode();
    payload.append(&mut who.encode());
    payload.append(&mut identity.encode());
    blake2_256(payload.as_slice()).to_vec()
}

pub trait ValidationDataBuilder {
    fn build_vdata_substrate(
        pair: &SubstratePair,
        who: &Address32,
        identity: &Identity,
        code: &ChallengeCode,
    ) -> ValidationData;
}

impl ValidationDataBuilder for ValidationData {
    fn build_vdata_substrate(
        pair: &SubstratePair,
        who: &Address32,
        identity: &Identity,
        challenge_code: &ChallengeCode,
    ) -> ValidationData {
        let message = get_expected_raw_message(who, identity, challenge_code);
        let sr25519_sig = pair.sign(&message);
        let signature = IdentityMultiSignature::Sr25519(sr25519_sig);
        let message = ValidationString::try_from(message).unwrap();

        let web3_common_validation_data = Web3CommonValidationData { message, signature };
        ValidationData::Web3(Web3ValidationData::Substrate(web3_common_validation_data))
    }
}

pub fn print_passed() {
    println!(" 🎉 All testcases passed!");
}

pub fn print_failed(reason: String) {
    println!(" ❌ Testcase failed, reason: {}", reason);
}
