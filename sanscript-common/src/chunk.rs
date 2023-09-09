use crate::value::ValueArray;
use crate::value::Value;
use strum_macros::Display;

#[repr(u8)]
#[derive(Copy, Clone, Display, Debug)]
pub enum OpCode {
    #[strum(serialize = "OP_RETURN")]
    OpReturn,
    #[strum(serialize = "OP_CONSTANT")]
    OpConstant(usize),
    #[strum(serialize = "OP_NEGATE")]
    OpNegate,
    #[strum(serialize = "OP_ADD")]
    OpAdd,
    #[strum(serialize = "OP_SUBTRACT")]
    OpSubtract,
    #[strum(serialize = "OP_MULTIPLY")]
    OpMultiply,
    #[strum(serialize = "OP_DIVIDE")]
    OpDivide,
    #[strum(serialize = "OP_TRUE")]
    OpTrue,
    #[strum(serialize = "OP_FALSE")]
    OpFalse,
    #[strum(serialize = "OP_NIL")]
    OpNil,
    #[strum(serialize = "OP_NOT")]
    OpNot,
    #[strum(serialize = "OP_EQUAL")]
    OpEqual,
    #[strum(serialize = "OP_GREATER")]
    OpGreater,
    #[strum(serialize = "OP_LESS")]
    OpLess
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
