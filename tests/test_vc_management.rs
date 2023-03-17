use litentry_test_suit::{get_shard, primitives::{Assertion, ParameterString, AssertionNetworks, Network}, vc_management::{api::{request_vc, build_request_vc_extrinsic, send_extrinsic}, events::wait_vc_issued_event}, identity_management::lit_set_user_shielding_key::set_user_shielding_key, API};

/**
 * Request VC Workflow
 */
#[test]
fn tc_request_vc() {
    // pre-condition
    set_user_shielding_key();
    let shard = get_shard();
    
    // inputs
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
        request_vc(shard, assertion);

        // Wait event
        let event = wait_vc_issued_event();
        println!(" ✅ [VCRequest] VC Index : {:?}", event.vc_index);
    });
}


#[test]
/**
 * Batch_All Request VC
 */
pub fn tc_batch_all_request_vc() {
    set_user_shielding_key();
    let shard = get_shard();
    
    let balance = 1_u128;
    let a4 = Assertion::A4(balance);
    let a7 = Assertion::A7(balance);
    let a10 = Assertion::A10(balance);
    let a11 = Assertion::A11(balance);

    let assertions = [a4, a7, a10, a11];
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        let xt = build_request_vc_extrinsic(shard, assertion);
    
        assertion_calls.push(xt.function);
    });

    let batching = API.batch(assertion_calls);

    let hex_encode_batching = batching.hex_encode();
    send_extrinsic(hex_encode_batching);
}