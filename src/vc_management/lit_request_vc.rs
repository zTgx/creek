use crate::{API, get_shard, primitives::{AesOutput, Assertion, ParameterString, Network, AssertionNetworks, MrEnclave}};
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus, StaticEvent};
use sp_core::H256;
use sp_core::{crypto::AccountId32 as AccountId};
use codec::Decode;
use std::{sync::mpsc::channel, thread};

pub fn tc_vm_00_request_vc() {
    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
    let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
    let balance = 1_u128;
    let litentry = Network::try_from("litentry".as_bytes().to_vec()).unwrap();
    let mut networks = AssertionNetworks::with_bounded_capacity(1);
    networks.try_push(litentry).unwrap();

    let a1 = Assertion::A1;
    let a2 = Assertion::A2(guild_id.clone());
    let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());
    let a4 = Assertion::A4(balance);
    let a6 = Assertion::A6;
    let a7 = Assertion::A7(balance);
    let a8 = Assertion::A8(networks);
    let a10 = Assertion::A10(balance);
    let a11 = Assertion::A11(balance);

    let shard = get_shard();

    // let assertions = [a1, a2, a3, a4, a6, a7, a8, a10, a11];
    let assertions = [a1];
    assertions.iter().for_each(|a| {
        request_vc(shard, &a);
    });
}

fn request_vc(shard: MrEnclave, assertion: &Assertion) {
    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "VCManagement",
        "request_vc",
        H256::from(shard),
        assertion
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    #[derive(Decode, Debug)]
    struct VCIssuedEvent {
        pub account: AccountId,
        pub vc_index: H256,
        pub vc: AesOutput,
    }

    impl StaticEvent for VCIssuedEvent {
        const PALLET: &'static str = "VCManagement";
        const EVENT: &'static str = "VCIssued";
    }

    let api2 = API.clone();
    let thread_output = thread::spawn(move || {
        let (events_in, events_out) = channel();
        api2.subscribe_events(events_in).unwrap();
        let args: VCIssuedEvent =
            api2.wait_for_event::<VCIssuedEvent>(&events_out).unwrap();
        args
    });

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);

    let args = thread_output.join().unwrap();
    println!("  [RequestVC] event: {:?}", args);    
}