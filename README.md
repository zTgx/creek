# litentry-test-suit ![](https://tokei.rs/b1/github/zTgx/litentry-test-suit)

# Unofficial
Although this is not an official repo, but the libraries(mainly refers to [substrate-api-client](https://github.com/scs/substrate-api-client)) that used in this repo will strictly consistent with [litentry-parachain](https://github.com/litentry/litentry-parachain).

```shell
cargo test --test test_vc_management --release -- --nocapture --test-threads=1
```

### Process of adding test cases
1. Design a testcase and add it to the form in [testcases](./docs/Testcases.md) format.  
2. Go to [tests](./tests/) to find the test file of the corresponding module. According to the principle of non-interdependence, refer to other cases and start from the `set_user_shielding_key` method to implement this newly designed case from scratch. 

### TODOs
- [ ] query-related api
  - [ ] identity-related query api
    - [x] `fn delegatee`
- [ ] decrypt id_graph
- [ ] cover identity-related event & error
- [ ] cover vc-related event & error
