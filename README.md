# litentry-test-suit ![](https://tokei.rs/b1/github/zTgx/litentry-test-suit)

# Unofficial

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
