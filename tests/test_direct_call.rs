use litentry_api_client::{
    api_client_patch::{event::SubscribeEventPatch, parachain::ParachainPatch},
    identity_management::{events::IdentityCreatedEvent, IdentityManagementApi},
    primitives::identity::{Identity, SubstrateNetwork},
    sidechain::{storage_key_challenge_code, SidechainRpc},
    utils::{
        address::pubkey_to_address32,
        crypto::{decrypt_challage_code_with_user_shielding_key, generate_user_shielding_key},
        enclave::mrenclave_to_bs58,
        print_passed,
    },
    ApiClient, direct_call::{top::{TrustedOperation, DirectCall}, trusted_call_signed::TrustedCall},
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_direct_call_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();
    let shard = api_client.get_shard().unwrap();

    let user_shielding_key = generate_user_shielding_key();

    let top: TrustedOperation =
    TrustedCall::set_user_shielding_key(alice.public().into().clone(), alice.public().into().clone(), user_shielding_key, Default::default())
    .sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &shard, &shard)
    .into();
    let res = api_client.send_request_di(&top);
    
    println!("Sidechain supported methods: {:?}", methods);
}
