/**
 * Including the verification of CI errors
 *
 * Format:
 * 1. comments: CI error url and error log
 * 2. add test function name: tc_ci_pr name_job
 *
 */
use litentry_api_client::{
    api_client_patch::{
        batch_all::BatchPatch, event::SubscribeEventPatch, parachain::ParachainPatch,
    },
    identity_management::{
        events::IdentityCreatedEvent, xtbuilder::IdentityManagementXtBuilder, IdentityManagementApi,
    },
    primitives::{
        address::Address32,
        identity::{Identity, Web2Network},
        MrEnclave,
    },
    utils::{address::public_to_address32, crypto::generate_user_shielding_key, print_passed},
    ApiClient,
};
use sp_core::{sr25519, Pair};
use sp_runtime::BoundedVec;

/*
CI ERROR： lit_batch_test
https://github.com/litentry/litentry-parachain/actions/runs/4447171033/jobs/7809442449?pr=1475

[2023-03-17T12:56:54Z DEBUG ita_stf::trusted_call] create_identity_runtime, who: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, identity: Web2 { network: Twitter, address: BoundedVec([109, 111, 99, 107, 95, 117, 115, 101, 114, 50], 64) }, metadata: None
[2023-03-17T12:56:54Z DEBUG ita_stf::trusted_call_litentry] create identity runtime, who = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", identity = Web2 { network: Twitter, address: BoundedVec([109, 111, 99, 107, 95, 117, 115, 101, 114, 50], 64) }, metadata = None, bn = 24, parent_ss58_prefix = 131
[2023-03-17T12:56:54Z DEBUG ita_stf::trusted_call_litentry] challenge code generated, who = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
[2023-03-17T12:56:54Z DEBUG ita_stf::trusted_call] create_identity_runtime d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d OK
[2023-03-17T12:56:54Z DEBUG enclave_runtime::top_pool_execution] Aura executed successfully
[2023-03-17T12:56:54Z INFO  enclave_runtime::top_pool_execution] Remaining time in slot (id: Slot(279842969), stage After AURA): 5556 ms, 92.60000000000001% of slot time
[2023-03-17T12:56:54Z DEBUG enclave_runtime::top_pool_execution] Proposing 1 sidechain block(s) (broadcasting to peers)
Syncing Parentchain block number 24 at Sidechain block number  42
[2023-03-17T12:56:54Z INFO  integritee_service::ocall_bridge::sidechain_ocall] Enclave produced sidechain blocks: [42]
[2023-03-17T12:56:54Z DEBUG integritee_service::ocall_bridge::sidechain_ocall] Updating peers..
[2023-03-17T12:56:54Z INFO  integritee_service::ocall_bridge::sidechain_ocall] Successfully updated peers
[2023-03-17T12:56:54Z DEBUG integritee_service::ocall_bridge::sidechain_ocall] Broadcasting sidechain blocks ...
[2023-03-17T12:56:54Z INFO  integritee_service::ocall_bridge::sidechain_ocall] Successfully broadcast blocks
[2023-03-17T12:56:54Z INFO  itp_extrinsics_factory] Creating extrinsics using nonce: 28
[2023-03-17T12:56:54Z DEBUG integritee_service::worker] Broadcasting block to peer with address: "ws://integritee-worker-2:2102"
[2023-03-17T12:56:54Z INFO  itp_extrinsics_factory] Creating extrinsics using nonce: 29
[2023-03-17T12:56:54Z DEBUG enclave_runtime::top_pool_execution] Sending sidechain block(s) confirmation extrinsic..
[2023-03-17T12:56:54Z DEBUG integritee_service::ocall_bridge::bridge_api] Requesting WorkerOnChain OCall API instance
[2023-03-17T12:56:54Z DEBUG integritee_service::ocall_bridge::worker_on_chain_ocall] Enclave wants to send 2 extrinsics
[2023-03-17T12:56:54Z DEBUG integritee_service::worker] Broadcasting block to peer with address: "ws://integritee-worker-1:2101"
[2023-03-17T12:56:54Z DEBUG integritee_service::ocall_bridge::worker_on_chain_ocall] Send extrinsic, call length: 454
[2023-03-17T12:56:54Z ERROR integritee_service::ocall_bridge::worker_on_chain_ocall] Could not send extrsinic to node: RpcClient(Extrinsic("extrinsic error code 1002: Verification Error: Runtime error: Execution failed: Execution aborted due to trap: wasm trap: wasm `unreachable` instruction executed\nWASM backtrace:\n\n    0: 0x4a563b - <unknown>!rust_begin_unwind\n    1: 0x13d59 - <unknown>!core::panicking::panic_fmt::h246cf959bff81f47\n    2: 0x2eae76 - <unknown>!TaggedTransactionQueue_validate_transaction\n: RuntimeApi(\"Execution failed: Execution aborted due to trap: wasm trap: wasm `unreachable` instruction executed\\nWASM backtrace:\\n\\n    0: 0x4a563b - <unknown>!rust_begin_unwind\\n    1: 0x13d59 - <unknown>!core::panicking::panic_fmt::h246cf959bff81f47\\n    2: 0x2eae76 - <unknown>!TaggedTransactionQueue_validate_transaction\\n\")"))
 */
#[test]
fn tc_ci_pr1475_7809442449() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice.clone()).unwrap();

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let who = public_to_address32(&alice.public());

    struct IdentityItem {
        pub shard: MrEnclave,
        pub who: Address32,
        pub identity: Identity,
        pub ciphertext_metadata: Option<Vec<u8>>,
    }

    let network = Web2Network::Twitter;
    let address =
        BoundedVec::try_from(vec![109, 111, 99, 107, 95, 117, 115, 101, 114, 50]).unwrap();
    let identity = Identity::Web2 { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let id1 = IdentityItem {
        shard: shard.clone(),
        who: who.clone(),
        identity,
        ciphertext_metadata,
    };

    let network = Web2Network::Twitter;
    let address = BoundedVec::try_from(vec![109, 111, 99]).unwrap();
    let identity = Identity::Web2 { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let id2 = IdentityItem {
        shard: shard.clone(),
        who: who.clone(),
        identity,
        ciphertext_metadata,
    };

    let ids = [id1, id2];
    let mut calls = vec![];
    ids.into_iter().for_each(|item| {
        calls.push(
            api_client
                .build_extrinsic_create_identity(
                    &item.shard,
                    &item.who,
                    &item.identity,
                    &item.ciphertext_metadata,
                )
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let event = api_client.wait_event::<IdentityCreatedEvent>();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    print_passed();
}
