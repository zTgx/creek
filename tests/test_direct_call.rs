use litentry_api_client::{
    api_client_patch::parachain::ParachainPatch,
    direct_call::{
        primitives::TrustedGetter,
        top::{DirectCall, TrustedOperation},
        trusted_call_signed::TrustedCall,
        types::{AccountId, KeyPair},
    },
    utils::{
        crypto::{generate_user_shielding_key, to_user_shielding_key_type},
        di::decode_user_shielding_key,
    },
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_di_set_user_shielding_key_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice.clone()).unwrap();
    let shard = api_client.get_shard().unwrap();

    let account: AccountId = alice.clone().public().into();

    let user_shielding_key = generate_user_shielding_key();
    println!(
        "Current user shielding key: {}",
        hex::encode(user_shielding_key.clone())
    );

    let nonce = 0_u32;

    let top: TrustedOperation = TrustedCall::set_user_shielding_key(
        alice.public().into(),
        alice.public().into(),
        to_user_shielding_key_type(&user_shielding_key),
        Default::default(),
    )
    .sign(
        &KeyPair::Sr25519(Box::new(alice.clone())),
        nonce,
        &shard,
        &sp_core::H256::from(shard),
    )
    .into();
    let _ = api_client.send_request_di(&top);

    // TODO:query
    println!("DI: Query UserShieldingKey>>>");
    let top: TrustedOperation = TrustedGetter::user_shielding_key(account)
        .sign(&KeyPair::Sr25519(Box::new(alice)))
        .into();
    let resp = api_client.send_request_di(&top).unwrap();
    let key = &resp.result[2..72];
    println!("key: {}", key);
    let decode_key = decode_user_shielding_key(key).unwrap();
    assert_eq!(hex::encode(user_shielding_key.clone()), decode_key);
}
