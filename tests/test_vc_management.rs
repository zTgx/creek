use litentry_test_suit::{
    identity_management::api::*,
    primitives::{Assertion, AssertionNetworks, Network, ParameterString},
    vc_management::{api::*, events::VcManagementEventApi, xtbuilder::VcManagementXtBuilder},
    ApiClient, ApiClientPatch, USER_AES256G_KEY,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    // Inputs
    let a1 = Assertion::A1;

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let a2 = Assertion::A2(guild_id.clone());

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
    let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
    let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());

    let balance = 10_u128;
    let a4 = Assertion::A4(balance);

    let a6 = Assertion::A6;

    let balance = 10_u128;
    let a7 = Assertion::A7(balance);

    let litentry = Network::try_from("litentry".as_bytes().to_vec()).unwrap();
    let mut networks = AssertionNetworks::with_bounded_capacity(1);
    networks.try_push(litentry).unwrap();
    let a8 = Assertion::A8(networks);

    let balance = 10_u128;
    let a10 = Assertion::A10(balance);

    let balance = 10_u128;
    let a11 = Assertion::A11(balance);

    let assertions = vec![a1, a2, a3, a4, a6, a7, a8, a10, a11];
    assertions.into_iter().for_each(|assertion| {
        api_client.request_vc(shard, assertion);

        // Wait event
        let event = api_client.wait_event_vc_issued();
        println!(" ✅ [VCRequest] VC Index : {:?}", event.vc_index);
    });
}

#[test]
pub fn tc_batch_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    let balance = 1_u128;
    let a4 = Assertion::A4(balance);
    let a7 = Assertion::A7(balance);
    let a10 = Assertion::A10(balance);
    let a11 = Assertion::A11(balance);

    let assertions = [a4, a7, a10, a11];
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        assertion_calls.push(
            api_client
                .build_extrinsic_request_vc(shard, assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.api.batch(assertion_calls).hex_encode());
}

#[test]
pub fn tc_batch_all_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    let balance = 1_u128;
    let a4 = Assertion::A4(balance);
    let a7 = Assertion::A7(balance);
    let a10 = Assertion::A10(balance);
    let a11 = Assertion::A11(balance);

    let assertions = [a4, a7, a10, a11];
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        assertion_calls.push(
            api_client
                .build_extrinsic_request_vc(shard, assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(assertion_calls).hex_encode());
}
