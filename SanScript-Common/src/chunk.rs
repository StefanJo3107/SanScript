use crate::value::ValueArray;
use crate::value::Value;
use core::fmt;

#[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize)
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpConstant(_) => write!(f, "OP_CONSTANT")
        }
    }
}

pub struct Chunk {
    code: Vec<OpCode>,
    constants: ValueArray,
    lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![], constants: ValueArray::new(), lines: vec![] }
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn get_code(&self, index: usize) -> &OpCode {
        &self.code[index]
    }

    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn get_constant(&self, offset: usize) -> &Value{
        self.constants.get(offset)
    }

    pub fn add_constant(&mut self, constant: Value) -> usize{
        self.constants.write_constant(constant);
        self.constants.len() - 1
    }

    pub fn get_line(&self, offset: usize) -> usize{
        self.lines[offset]
    }
}
