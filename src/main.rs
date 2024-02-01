use mini_revm::evm::EVM;
use mini_revm::parser::Parser;

fn main() {
    println!("Hello, mini EVM!");
    let mut parser = Parser::new();
    let instructions = parser.parse(&String::from("6002600201"));
    let mut evm = EVM::new();
    evm.execute_byte_code(&instructions);
    evm.print_stack();
}
