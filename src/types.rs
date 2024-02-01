use std::ops;
use num_bigint::BigUint;
use evm::Opcode;
#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd)]
pub struct Bytes32(pub Vec<u8>);

impl Bytes32{
    pub fn new() -> Self{
        Bytes32(Vec::<u8>::new())
    }
    pub fn from_bytes(b: Vec<u8>) -> Self{
        Bytes32(b.to_vec())
    }
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0)
    }
    pub fn from_biguint(b: BigUint) -> Self {
        Bytes32(b.to_bytes_be())
    }
}

impl ops::Mul for Bytes32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a * b;
        Bytes32(BigUint::to_bytes_be(&sum))
    }
}

impl ops::Add for Bytes32{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a + b;
        Bytes32(BigUint::to_bytes_be(&sum))
    } 
}

impl ops::Sub for Bytes32{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a - b;
        Bytes32(BigUint::to_bytes_be(&sum))
    } 
}

impl ops::Div for Bytes32{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a / b;
        Bytes32(BigUint::to_bytes_be(&sum))
    } 
}

impl ops::BitAnd for Bytes32{
    type Output = Self;
    fn bitand(self, other: Self) -> Self{
        let a = BigUint::from_bytes_be(self.0.as_slice());
        let b = BigUint::from_bytes_be(other.0.as_slice());
        let res = a & b;
        Bytes32(BigUint::to_bytes_be(&res))
    }
}

impl ops::BitOr for Bytes32{
    type Output = Self;
    fn bitor(self, other: Self) -> Self{
        let a = BigUint::from_bytes_be(self.0.as_slice());
        let b = BigUint::from_bytes_be(other.0.as_slice());
        let res = a | b;
        Bytes32(BigUint::to_bytes_be(&res))
    }
}

impl ops::BitXor for Bytes32 {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());
        let b = BigUint::from_bytes_be(other.0.as_slice());
        let res = a ^ b;
        Bytes32(BigUint::to_bytes_be(&res))
    }
}

impl ops::Not for Bytes32 {
    type Output = Self;
    fn not(self:Self) -> Self {
        let negated_bytes: Vec<u8> = self.0.iter().map(|&byte| !byte).collect();
        Bytes32(negated_bytes) 
    }
}

impl ops::Shl for Bytes32 {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self {
        // todo 
        Bytes32::new()
    }
}

impl ops::Shr for Bytes32 {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self{
        // todo 
        Bytes32::new()
    }
}


type PushData = String;
pub type Insc = (Opcode,Option<PushData>);

#[cfg(test)]
mod tests {
    #[test]
    fn test_mul() {
        
    }
}