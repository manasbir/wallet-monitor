use ethers::contract::Abigen;

fn main() {
    println!("building!");
    let erc20 = Abigen::new("ERC20", "./abis/abi.json")
        .unwrap()
        .generate()
        .unwrap();
    erc20.write_to_file("./src/bindings/erc20.rs").unwrap();
}