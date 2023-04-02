use crate::{
    primitives::address::{Address20, Address32},
    ACCOUNT_SEED_CHARSET,
};
use aes_gcm::aead::OsRng;
use rand::{Rng, RngCore};
use sp_core::{sr25519, Pair};

/// TODO:
/// refacotr function name
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

pub fn vec_to_u8_32_array(input: Vec<u8>) -> [u8; 32] {
    assert_eq!(input.len(), 32);

    let mut bytes = [0u8; 32];
    bytes[..32].clone_from_slice(&input);

    bytes
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
