use creek::{Creek, WorkerPublicApis, WorkerTxApi};

fn main() {
	let creek = Creek::new();
	let _ = creek.author_get_enclave_signer_account();

	creek.link_identity();
}
