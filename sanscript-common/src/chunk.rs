use crate::value::ValueArray;
use crate::value::Value;
use strum_macros::Display;

#[repr(u8)]
#[derive(Copy, Clone, Display, Debug)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpTrue,
    OpFalse,
    OpNil,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop
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
