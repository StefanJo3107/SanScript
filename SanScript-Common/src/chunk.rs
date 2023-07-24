use crate::value;
use core::fmt;

#[repr(u8)]
pub enum OpCode {
    OpReturn,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
        }
    }
}

pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn get(&self, index: usize) -> &OpCode {
        &self.code[index]
    }

    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}
