* Bugs are being catched will list here by using this helper.

1. Using empty params to request direct server, `worker` will crash.
```rust
// fn request_vc(&self, assertion: Assertion) -> CResult<()>

let params: Vec<String> = vec![];
let jsonreq = json_req("author_submitAndWatchAesRequest", params, 1);
```
Reason: Out of range index, the `hex_encoded_params` is empty, index 0 will be out of range.  
https://github.com/litentry/litentry-parachain/blob/01d364ed820212deafe9b38ca53c59a9d3f85d80/tee-worker/sidechain/rpc-handler/src/direct_top_pool_api.rs#L334

