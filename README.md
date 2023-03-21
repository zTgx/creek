# litentry-test-suit ![](https://tokei.rs/b1/github/zTgx/litentry-test-suit)

# Unofficial
Although this is not an official repo, but the libraries(mainly refers to [substrate-api-client](https://github.com/scs/substrate-api-client)) that used in this repo will strictly consistent with [litentry-parachain](https://github.com/litentry/litentry-parachain).

### TODO
- [x] identity verify
- [x] decrypt identity
- [x] decrypt challenge_code
- [ ] decrypt id_graph
- [ ] query sidechain storage
- [x] build_vdata_substrate
- [ ] query-related api
  - [x] identity-related query api
    - [x] `fn delegatee`
  - [ ] vc-related query api
    - [x] `vc_registry`
    - [ ] `schema_admin`
    - [ ] `schema_index`
    - [ ] `schema_registry`
- [ ] cover identity-related event & error
  - [ ] events
    - [x] UnexpectedMessage
    - [x] UserShieldingKeySet
    - [x] SetUserShieldingKeyHandlingFailed
    - [x] IdentityCreated
    - [x] IdentityRemoved
    - [x] IdentityVerified
    - [x] DelegateeAdded
    - [x] IdentityVerified
  - [ ] errors
    - [x] UnauthorisedUser
    - [ ] DelegateeNotExist 
- [ ] cover vc-related event & error
  - [ ] events
    - [x] VCIssued
    - [x] VCDisabled
    - [x] VCRevoked
- [ ] subscribe system events
  - [ ] ExtrinsicSuccess
  - [ ] ExtrinsicFailed

---

### Process of adding test cases
1. Design a testcase and add it to the form in [testcases](./docs/Testcases.md) format.  
2. Go to [tests](./tests/) to find the test file of the corresponding module. According to the principle of non-interdependence, refer to other cases and start from the `set_user_shielding_key` method to implement this newly designed case from scratch. 
