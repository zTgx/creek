use litentry_api_client::{
    api_client_patch::parachain::ParachainPatch,
    direct_call::{
        top::{DirectCall, TrustedOperation},
        trusted_call_signed::TrustedCall,
        types::KeyPair,
    },
    utils::crypto::{generate_user_shielding_key, to_user_shielding_key_type},
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_di_set_user_shielding_key_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice.clone()).unwrap();
    let shard = api_client.get_shard().unwrap();

    let user_shielding_key = generate_user_shielding_key();

    let nonce = 0_u32;

    let top: TrustedOperation = TrustedCall::set_user_shielding_key(
        alice.public().into(),
        alice.public().into(),
        to_user_shielding_key_type(&user_shielding_key),
        Default::default(),
    )
    .sign(
        &KeyPair::Sr25519(Box::new(alice)),
        nonce,
        &shard,
        &sp_core::H256::from(shard),
    )
    .into();
    api_client.send_request_di(&top);
}
