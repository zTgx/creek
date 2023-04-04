# litentry-api-client [![](https://img.shields.io/tokei/lines/github/zTgx/litentry-api-client?style=flat-square)](https://img.shields.io/tokei/lines/github/zTgx/litentry-api-client?style=for-the-badge)

### Features
- [x] Rust nightly only
- [x] Based on [substrate-api-client](https://github.com/scs/substrate-api-client) and `ApiClientPatch`
- [x] `IdentityManagement` pallet api is supported
- [x] `VCManagement` pallet api is supported
- [x] `VC verify` is supported
- [x] `Sidechain` api is supported
- [x] `RA Attestation` is supported

### Usage
```rust
fn set_user_shielding_key() {
    // 1. Create Api client with signer
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    // 2. Setting user shielding key
    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    // 3. Wait event
    let event = api_client.wait_event_user_shielding_key_set();
    assert!(event.is_ok());
}
```
