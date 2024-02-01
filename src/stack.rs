use crate::types::Bytes32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    stack: Vec<Bytes32>,
}

impl Stack{
    pub fn new() -> Self{
        Stack{
            stack: Vec::<Bytes32>::new(),
        }
    }

    pub fn push(&mut self, input:Bytes32){
        if self.stack.len() > 1024{
            panic!("stack overflow")
        }
        self.stack.push(input);
    }

    pub fn pop(&mut self) -> Bytes32{
        if self.stack.len() < 1{
            panic!("stack underflow")
        }
        self.stack.pop().unwrap_or(Bytes32::new())
    }

}