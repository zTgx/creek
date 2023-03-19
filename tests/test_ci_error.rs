/**
 * Including the verification of CI errors
 *
 * Format:
 * 1. comments: CI error url and error log
 * 2. add test function name: tc_ci_pr name_job
 *
 */
use litentry_test_suit::{
    identity_management::{api::IdentityManagementApi, events::IdentityManagementEventApi},
    primitives::{Address32, Identity, Web2Network},
    utils::print_passed,
    ApiClient, USER_AES256G_KEY,
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
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    let network = Web2Network::Twitter;
    let address =
        BoundedVec::try_from(vec![109, 111, 99, 107, 95, 117, 115, 101, 114, 50]).unwrap();

    let identity = Identity::Web2 { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    // Alice
    let add =
        hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    let mut y = [0u8; 32];
    y[..32].clone_from_slice(&add);
    let who = Address32::from(y);

    api_client.create_identity(shard, who, identity, ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert_eq!(event.who, api_client.get_signer().unwrap());

    print_passed();
}
