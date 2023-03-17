# litentry-test-suit ![](https://tokei.rs/b1/github/zTgx/litentry-test-suit)

# Unofficial
Although this is not an official repo, but the libraries(mainly refers to [substrate-api-client](https://github.com/scs/substrate-api-client)) that used in this repo will strictly consistent with [litentry-parachain](https://github.com/litentry/litentry-parachain).

---

### Run all test
```shell
./scripts/test_all.sh
```

### Run specific tc
```shell
cargo test --test test_identity_management tc_00 --release -- --nocapture
```

### Write a new testcase?
First, add a new test case in [testcases](./docs/Testcases.md)  
Secondly, impl it in [rust](./src/)   
Last, impl [tests](./tests/)  
