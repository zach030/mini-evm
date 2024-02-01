use crate::types::Insc;
use evm::Opcode;
use std::i64;

const OP_SIZE:usize = 2;

pub struct Parser {
    instructions: Vec<Insc>,
}

impl Parser {
    pub fn new()->Parser{
        Parser{
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self, bytecode:&String) -> &Vec<Insc> {
        let code_size = bytecode.len();
        if code_size==0{
            panic!("code_size is zero");
        }
        let mut i=0;
        while i< code_size{
            let op = Opcode{
                0: i64::from_str_radix(&bytecode[i..i+OP_SIZE], 16).unwrap() as u8,
            };
            println!("op:{}",op.as_u8());
            i+=OP_SIZE;
            // todo handle other opcode value
            let mut push_data_size:usize=0;
            // op.is_push return push bytes size, eg. 1,2,3, data size is two times of bytes size
            if let Some(len) =  op.is_push(){
                push_data_size = (len*2) as usize;
            }
            if push_data_size==0{
                self.instructions.push(
                    (
                        Opcode{0: op.0,},None
                    )
                );
            }else{
                let push_data = &bytecode[i..i+push_data_size];
                println!("push_data: {}",push_data);
                self.instructions.push(
                    (
                        Opcode{0:op.0}, Some(push_data.to_string())
                    )
                );
                i+=push_data_size;
            }
        }
        &self.instructions
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcodes, parser::Parser};
    #[test]
    fn it_works() {
        let mut parser = Parser::new();
        let ins = parser.parse(&String::from("60026001"));
        assert_eq!(ins[0].0.as_u8(), opcodes::PUSH1);
        assert_eq!(ins[0].clone().1.unwrap(), String::from("02"));

        assert_eq!(ins[1].0.as_u8(), opcodes::PUSH1);
        assert_eq!(ins[1].clone().1.unwrap(), String::from("01"));
    }
}