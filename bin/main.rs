use creek::{Creek, WorkerPublicApis};

fn main() {
	let creek = Creek::new();
	let _ = creek.author_get_enclave_signer_account();
}
