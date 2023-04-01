# litentry-test-suit ![](https://img.shields.io/tokei/lines/github/zTgx/litentry-test-suit?style=flat)

### Features
- [x] rust only
- [x] based on [substrate-api-client](https://github.com/scs/substrate-api-client) and `ApiClientPatch`
- [x] `IdentityManagement` pallet api is supported
- [x] `VCManagement` pallet api is supported
- [x] `VC verify` is supported
- [x] `sidechain` api is supported

```rust
#[test]
fn tc_set_user_shielding_key_works() {
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