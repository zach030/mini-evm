use num_bigint::BigUint;
use crate::opcodes;
use crate::stack;
use crate::types;
use std::convert::TryInto;
#[derive(Debug)]
pub struct EVM {
    stack: stack::Stack,
}

impl EVM{
    pub fn new() ->EVM{
        EVM{
            stack: stack::Stack::new(),
        }
    }

    pub fn execute_byte_code(&mut self, ins:&Vec<types::Insc>){
        for i in ins.iter(){
            match i.0.as_u8() {
                opcodes::PUSH0..=opcodes::PUSH32 => {
                    self.push( i.1.as_ref().unwrap());
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
                },
                opcodes::LT => {
                    self.lt();
                },
                opcodes::GT => {
                    self.gt();
                },
                opcodes::EQ => {
                    self.eq();
                },
                opcodes::AND => {
                    self.and();
                },
                opcodes::OR => {
                    self.or();
                },
                opcodes::XOR => {
                    self.xor();
                },
                opcodes::NOT => {
                    self.not();
                },
                opcodes::SHL => {
                    self.shl();
                },
                opcodes::SHR => {
                    self.shr();
                }
                _ =>{}
            }
        }
    }

    pub fn print_stack(&self) {
        println!("stack = {:?}", self.stack)
    }

    fn push(&mut self, data:&String){ 
        self.stack.push(types::Bytes32(hex::decode(data).unwrap()));
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
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![0])));
            ()
        }
        // todo
        self.stack.push(a/b);
    }

    fn lt(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        if b<a{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![1])));
        }else{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![0])));
        }
    }

    fn gt(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        if b>a{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![1])));
        }else{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![0])));
        }
    }

    fn eq(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        if b==a{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![1])));
        }else{
            self.stack.push(types::Bytes32::from_biguint(BigUint::new(vec![0])));
        }
    }

    fn and(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        self.stack.push(a&b)
    }

    fn or(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        self.stack.push(a|b)
    }

    fn xor(&mut self){
        let a= self.stack.pop();
        let b= self.stack.pop();
        self.stack.push(a^b)
    }

    fn not(&mut self){
        let a= self.stack.pop();
        self.stack.push(!a)
    }

    fn shl(&mut self){
        let a= self.stack.pop();
        self.stack.push(!a)
    }
    fn shr(&mut self){
        let a= self.stack.pop();
        self.stack.push(!a)
    }
}