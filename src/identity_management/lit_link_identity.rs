
const substrateIdentity = <LitentryIdentity>{
    Substrate: <SubstrateIdentity>{
        address: '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d', //Alice
        network: 'Litentry',
    },
};

const [resp_twitter, resp_ethereum, resp_substrate] = (await createIdentities(
    context,
    context.defaultSigner[0],
    aesKey,
    true,
    [twitterIdentity, ethereumIdentity, substrateIdentity]


    export async function createIdentities(
        context: IntegrationTestContext,
        signer: KeyringPair,
        aesKey: HexString,
        listening: boolean,
        identities: LitentryIdentity[]
    ): Promise<IdentityGenericEvent[] | undefined> {
        let txs: TransactionSubmit[] = [];
        for (let index = 0; index < identities.length; index++) {
            const identity = identities[index];
            const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
            const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');
            const tx = context.substrate.tx.identityManagement.createIdentity(
                context.mrEnclave,
                signer.address,
                `0x${ciphertext}`,
                null
            );
            const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
            let newNonce = nonce.toNumber() + index;
            txs.push({ tx, nonce: newNonce });
        }


fn create_identity() {

    let shard = get_shard();
    let signer = get_signer();
    let ciphertext = encrypt_with_tee_shielding_pubkey();
    
    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "create_identity",
        H256::from(shard),
        assertion
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    #[derive(Decode, Debug)]
    struct VCIssuedEvent {
        pub account: AccountId,
        pub vc_index: H256,
        pub vc: AesOutput,
    }

    impl StaticEvent for VCIssuedEvent {
        const PALLET: &'static str = "VCManagement";
        const EVENT: &'static str = "VCIssued";
    }

    let api2 = API.clone();
    let thread_output = thread::spawn(move || {
        let (events_in, events_out) = channel();
        api2.subscribe_events(events_in).unwrap();
        let args: VCIssuedEvent =
            api2.wait_for_event::<VCIssuedEvent>(&events_out).unwrap();
        args
    });

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);

    let args = thread_output.join().unwrap();
    println!("  [RequestVC] event: {:?}", args);    
}