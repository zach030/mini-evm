use crate::opcodes;
use crate::stack;
use crate::types;
use std::convert::TryInto;
#[derive(Debug)]
pub struct EVM {
    code: Vec<u8>,
    pc: u64,
    stack: stack::Stack,
}

impl EVM{
    pub fn new(code:Vec<u8>) ->EVM{
        EVM{
            code,
            pc:0,
            stack: stack::Stack::new(),
        }
    }
    fn next(&mut self) -> u8{
        let op = self.code[self.pc as usize];
        self.pc+=1;
        op
    }
    pub fn run(&mut self){
        while self.pc < self.code.len() as u64{
            let op = self.next();
            match op{
                opcodes::PUSH0..=opcodes::PUSH32 => {
                    self.push((op-opcodes::PUSH0) as u64);
                },
                opcodes::POP=>{
                    self.pop();
                },
                opcodes::ADD => {
                    self.add();
                },
                opcodes::SUB => {
                    self.sub();
                },
                opcodes::MUL => {
                    self.mul();
                },
                opcodes::DIV => {
                    self.div();
                },
                opcodes::SDIV => {
                    self.sdiv();
                }
               _ => {}
            }
        }
    }

    pub fn print_stack(&self) {
        println!("stack = {:?}", self.stack)
    }

    fn push(&mut self, size:u64){
        let slice = &self.code[self.pc as usize..(self.pc + size) as usize];        
        let hex_string = slice.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        print!("current op code: PUSH{}, push value: {:?}\n",size, hex_string);
        match hex::decode(&hex_string) {
            Ok(decoded) => {
                match decoded.try_into() {
                    Ok(bytes32) => {
                        self.stack.push(types::Bytes32(bytes32));
                    },
                    Err(_) => {
                        println!("Failed to convert to Bytes32");
                    }
                }
            },
            Err(e) => {
                println!("Hex decoding error: {:?}", e);
            }
        }
        self.pc += size;
    }

    fn pop(&mut self) -> types::Bytes32{
        self.stack.pop()
    }

    fn mul(&mut self){
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(a*b);
    }
    
    fn add(&mut self){
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(a+b);
    }

    fn sub(&mut self){
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(a-b);
    }

    fn div(&mut self){
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(a/b);
    }
    
    fn sdiv(&mut self){
        let a = self.stack.pop();
        let b = self.stack.pop();
        if b.is_zero(){
            self.stack.push(types::Bytes32("0".try_into().unwrap()));
            ()
        }
        // todo
        self.stack.push(a/b);
    }
}