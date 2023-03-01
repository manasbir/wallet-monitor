fn main() {
    let provider = Provider::try_from(format!("https://eth-goerli.g.alchemy.com/v2/{}", env::var("RPC").expect("RPC not set"))).unwrap();
    let key = hex::decode(env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set")).unwrap();
    let signer = SignerMiddleware::new(provider, key);

    let contract = Contract::new(
        "0x9696bc05C4E9B8992059915eA69EF4cDf391116B".parse().unwrap(),
        include_bytes!("../abis/abi.json"),
        signer,
    );
}
