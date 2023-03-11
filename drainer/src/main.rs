use std::{thread::sleep, env};

pub mod bindings { pub mod erc20;}
use bindings::erc20 as ERC20;

// need to call etherscan api for transfers of erc20, ether, and erc721
// track block numbers, until block #1679155200 // what the fuck?
// can use ethers-rs to get the block number and maybe track txns??
use ethers::{prelude::*, utils::hex};
use tokio;
use reqwest;
use std::time::Duration;
use dotenv::dotenv;
use serde_json::{Result, Value};

// Generate the `abigen!` macro using only the functions we need

trait IsFn {
    fn is_fn (&self, fn_sig: &str) -> bool;
}

impl IsFn for str {
    fn is_fn(&self, fn_sig: &str) -> bool {
        if self.to_lowercase().contains(fn_sig) {
            return true;
        } else {
            return false;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Hello, world!");
    tracker().await;
    //println!("Block number: {}, Block Info: {}", block_num, );
}

async fn tracker() -> Result<(), >{
    let provider = Provider::try_from(format!("https://eth-goerli.g.alchemy.com/v2/{}", env::var("RPC").expect("RPC not set"))).unwrap();
    let flashbots = 1;
    let signer_compromised = SignerMiddleware::new(provider, hex::decode(env::var("P_KEY_COMPRIMISED").expect("P_KEY_COMPRIMISED not set")).unwrap());
    let signer = SignerMiddleware::new(provider, hex::decode(env::var("P_KEY").expect("P_KEY not set")).unwrap());
    let address_str = "0x9696bc05C4E9B8992059915eA69EF4cDf391116B";
    let address: ethers::types::Address = address_str.parse().unwrap();
    let mut block_number = provider.get_block_number().await.unwrap();

    loop {
        println!("Block number: {}", block_number);
        // track etherscan and with alchemy
        if block_number == provider.get_block_number().await.unwrap() {
            sleep(Duration::from_secs(9));
            continue;
        } else {
            block_number = provider.get_block_number().await.unwrap();
        }

        let block_txs = provider.get_block_with_txs(block_number).await.unwrap();

        for tx in block_txs.unwrap().transactions {
            
            if tx.to == Some(address) || tx.from == address {
                let sender = resolve_ens_name(tx.from).await;
                let receiver = resolve_ens_name(tx.to.unwrap()).await;
                println!("From: {:?}, To: {:?}", sender, receiver);
            }
            if tx.input.to_string().to_lowercase().contains(&address_str.replace("0x", "").to_lowercase()) {
                if tx.input.to_string().is_fn(&"0xa9059cbb++") {
                    println!("{} transferred {} {:?} to you and now transfering out....", 
                        resolve_ens_name(tx.from).await,
                        &tx.input.to_string()[74..138].parse::<U256>().unwrap()/U256::from(10u64.pow(18)),
                        ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await.unwrap()
                    );
                        
                } 
                else {
                    let body = reqwest::get(format!("https://sig.eth.samczsun.com/api/v1/signatures?function={}", &tx.input.to_string()[0..10])).await.unwrap();  
                    let fn_name = body.json::<Value>().await.unwrap()["result"]["function"][&tx.input.to_string()[0..10]][0]["name"].as_str().unwrap();
                    // need error handling lmao


                    // with the four_byte we can use abigen decode to get the whole function
                    // abigen requires a contract abi

                    let res = reqwest::get(format!("https://api-goerli.etherscan.io/api?module=contract&action=getabi&address={:?}&apikey={}", &tx.to.unwrap(), env::var("E_KEY").expect("E_KEY not set"))).await.unwrap();
                    let abi = &res.json::<Value>().await.unwrap()["result"];
                    let abi = abi.as_str().unwrap();

                    let bindings = Abigen::new("unkown_abi", &abi).unwrap().generate().unwrap();
                    
                    // rip
                    
                    
                    // need to create a matching algorithm
                    // :(
                }
            }
        }

        sleep(Duration::from_secs(9));
    }
}

async fn resolve_ens_name(address: H160) -> String {
    let provider = Provider::try_from(format!("https://eth-goerli.g.alchemy.com/v2/{}", env::var("RPC").expect("RPC not set"))).unwrap();
    let name = provider.lookup_address(address).await;
    match name {
        Ok(name) => {
            return name;
        },
        Err(_) => {
            return address.to_string();
        }
    }
        
}