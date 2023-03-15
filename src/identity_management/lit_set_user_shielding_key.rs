use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use sp_core::H256;
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus, StaticEvent};
use crate::{get_shard, primitives::{AesOutput, Address32, SubstrateNetwork}, API, utils::encrypt_with_tee_shielding_pubkey, get_signer, LIT_Aes256G_KEY};
use sp_core::{crypto::AccountId32 as AccountId};
use codec::{Decode, Encode};
use std::{sync::mpsc::channel, thread};

pub fn set_user_shielding_key() {
    let aes_key = LIT_Aes256G_KEY.to_vec();
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

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

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

pub fn create_identity() {
    use crate::primitives::Identity;

    let add = hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    let mut y = [0u8; 32];
    y[..32].clone_from_slice(&add);

    let address = Address32::from(y);
    let network = SubstrateNetwork::Litentry;

    let identity = Identity::Substrate { network, address };
    
    let msg = identity.encode();
    let shard = get_shard();
    let signer = get_signer();
    let ciphertext = encrypt_with_tee_shielding_pubkey(&msg);
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "create_identity",
        H256::from(shard),
        signer,
        ciphertext,
        ciphertext_metadata
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}