# litentry-api-client ![](https://img.shields.io/tokei/lines/github/zTgx/litentry-api-client?style=flat-square) ![](https://img.shields.io/github/commit-activity/m/zTgx/litentry-api-client/main?style=flat-square)
**WIP**  
[Litentry Parachain](https://github.com/litentry/litentry-parachain) Rust version SDK.

### Features
- [x] Rust nightly only
- [x] Based on [substrate-api-client](https://github.com/scs/substrate-api-client)
- [x] Support `IdentityManagement` & `VCManagement` & `Sidechain` pallets
- [x] Support `VC verification` & `RA`

```rust
/// Set User Shielding Key And Wait Event

// 1. Create Api client with signer
let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
let api_client = ApiClient::new_with_signer(alice);

// 2. Setting user shielding key
let shard = api_client.get_shard().unwrap();
let user_shielding_key = generate_user_shielding_key();
api_client.set_user_shielding_key(&shard, &user_shielding_key);

// 3. Wait event
let event = api_client.wait_event::<SetUserShieldingKeyEvent>();
assert!(event.is_ok());
```
