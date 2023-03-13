use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use sp_core::H256;
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus, StaticEvent};
use crate::{get_shard, API, utils::encrypt_with_tee_shielding_pubkey, get_signer};
use sp_core::{crypto::AccountId32 as AccountId};
use codec::Decode;
use std::{sync::mpsc::channel, thread};

pub fn tc00_set_user_shielding_key() {
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    println!("  [SetUserShieldingKey]-TC00 aes_key: {:?}", aes_key);

    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);

    let shard = get_shard();
    println!("  [SetUserShieldingKey]-TC00 shard: {:?}", shard);

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key.to_vec()
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    #[derive(Decode)]
    struct SetUserShieldingKeyEventArgs {
        pub account: AccountId,
    }

    impl StaticEvent for SetUserShieldingKeyEventArgs {
        const PALLET: &'static str = "IdentityManagement";
        const EVENT: &'static str = "UserShieldingKeySet";
    }

	let api2 = API.clone();
	let thread_output = thread::spawn(move || {
		let (events_in, events_out) = channel();
		api2.subscribe_events(events_in).unwrap();
		let args: SetUserShieldingKeyEventArgs =
			api2.wait_for_event::<SetUserShieldingKeyEventArgs>(&events_out).unwrap();
		args
	});

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);

	let args = thread_output.join().unwrap();
	println!("Transactor: {:?}", args.account);

    assert_eq!(args.account, get_signer());
}

pub fn tc01_set_user_shielding_key() {
    let aes_key = [0, 1];
    println!("  [SetUserShieldingKey]-TC01 aes_key: {:?}", aes_key);
    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);

    let shard = get_shard();
    println!("  [SetUserShieldingKey]-TC01 shard: {:?}", shard);

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key.to_vec()
    );
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    #[derive(Decode)]
    struct SetUserShieldingKeyHandlingFailedEventArgs;

    impl StaticEvent for SetUserShieldingKeyHandlingFailedEventArgs {
        const PALLET: &'static str = "IdentityManagement";
        const EVENT: &'static str = "SetUserShieldingKeyHandlingFailed";
    }
	let api2 = API.clone();
	let thread_output = thread::spawn(move || {
		let (events_in, events_out) = channel();
		api2.subscribe_events(events_in).unwrap();
		let args: SetUserShieldingKeyHandlingFailedEventArgs =
			api2.wait_for_event::<SetUserShieldingKeyHandlingFailedEventArgs>(&events_out).unwrap();
		args
	});

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);

    assert_eq!(thread_output.join().is_ok(), true);

}