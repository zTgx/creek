use crate::{API, get_shard, primitives::{AesOutput, Assertion}};
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus, StaticEvent};
use sp_core::H256;
use sp_core::{crypto::AccountId32 as AccountId};
use codec::Decode;
use std::{sync::mpsc::channel, thread};

pub fn tc_vm_00_request_vc() {
    let assertion = Assertion::A1;
    let shard = get_shard();
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