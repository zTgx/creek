# litentry-test-suit
![](https://img.shields.io/github/commit-activity/m/zTgx/litentry-test-suit?style=for-the-badge)  ![](https://img.shields.io/github/languages/code-size/zTgx/litentry-test-suit?style=for-the-badge) ![](https://img.shields.io/tokei/lines/github/zTgx/litentry-test-suit?style=for-the-badge)

# Unofficial
Although this is not an official repo, but the libraries(mainly refers to [substrate-api-client](https://github.com/scs/substrate-api-client)) that used in this repo will strictly consistent with [litentry-parachain](https://github.com/litentry/litentry-parachain).

### Features
- [x] rust only
- [x] api is flexiable and easy to use
- [x] based on [substrate-api-client](https://github.com/scs/substrate-api-client) and `ApiClientPatch`
- [x] `IdentityManagement` pallet api is supported
- [x] `VCManagement` pallet api is supported
- [ ] `sidechain` api is supported

---

### A Demo
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
    let expect_event = SetUserShieldingKeyEvent {
        account: api_client.get_signer().unwrap(),
    };
    assert!(event.is_ok());
    assert_eq!(event.unwrap(), expect_event);
}
```

### Process of adding test cases
1. Design a testcase and add it to the form in [testcases](./docs/Testcases.md) format.  
2. Go to [tests](./tests/) to find the test file of the corresponding module. According to the principle of non-interdependence, refer to other cases and start from the `set_user_shielding_key` method to implement this newly designed case from scratch. 
