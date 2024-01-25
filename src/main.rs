use mini_revm::evm::EVM;

fn main() {
    println!("Hello, mini EVM!");
    let code_hex = "6003600604";
    let code_bytes = hex::decode(code_hex).expect("Invalid hex string");
    let mut evm = EVM::new(code_bytes);
    evm.run();
    evm.print_stack();
}
