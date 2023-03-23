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
- [x] `utils` methods is supported
- [x] covered [identity-management](./tests/test_identity_management.rs)
- [x] covered [vc-management](./tests/test_vc_management.rs)
- [x] covered [vc-verify](./tests/test_vc_verify.rs)
- [x] covered [corner-case](./tests/test_corner_case.rs)
- [x] covered [ci-error](./tests/test_ci_error.rs)
- [x] covered all events & errors

### Todo
- [x] decrypt identity
- [x] decrypt challenge_code
- [ ] decrypt id_graph
- [ ] query sidechain storage
- [x] build_vdata_substrate
- [ ] event timeout
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
