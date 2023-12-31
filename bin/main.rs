use creek::{
	primitives::{
		identity::{Identity, IdentityString},
		keypair::KeyPair,
		network::Web3Network,
	},
	Creek, ValidationDataBuilder, WorkerSTF,
};
use sp_core::{sr25519, Pair};

fn main() {
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
	let creek = Creek::new_with_signer(KeyPair::from(alice));

	// Web2 Identity
	let twitter_identity = Identity::Twitter(IdentityString::new("mock_user".as_bytes().to_vec()));
	let vdata = creek.twitter_vdata("twitterid").unwrap();
	let _ = creek.link_identity(twitter_identity, vec![], vdata);

	// Web3 Identity
	let bob = sr25519::Pair::from_string("//Bob", None).unwrap();
	let vdata = creek.web3_vdata(&KeyPair::from(bob.clone())).unwrap();
	let bob_identity = Identity::Substrate(bob.public().into());
	let _ = creek.link_identity(bob_identity, vec![Web3Network::Litentry], vdata);
}
