use creek::{Creek, WorkerSTF, primitives::{keypair::KeyPair, identity::{Identity, IdentityString}}};
use sp_core::{sr25519, Pair};

fn main() {
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
	let creek = Creek::new_with_signer(KeyPair::from(alice));

	// Web2 Identity
	let twitter_identity = Identity::Twitter(IdentityString::new("mock_user".as_bytes().to_vec()));
	let _ = creek.link_identity(twitter_identity, vec![]);

// 	// Web3 Identity
// 	let bob = sr25519::Pair::from_string("//Bob", None).unwrap();
// 	let bob_identity = Identity::Substrate(bob.public().into());
	// let _ = creek.link_identity(bob_identity, vec![Web3Network::Litentry]);
}
