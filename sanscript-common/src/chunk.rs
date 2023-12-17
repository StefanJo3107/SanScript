use crate::value::ValueArray;
use crate::value::Value;
use strum_macros::Display;

#[repr(u8)]
#[derive(Copy, Clone, Display, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
    OpDefineGlobal(usize),
    OpGetGlobal(usize),
    OpSetGlobal(usize),
    OpGetLocal(usize),
    OpSetLocal(usize),
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
    OpPop,
    OpJumpIfFalse(usize),
    OpJump(usize)
}

pub struct Chunk {
    code: Vec<OpCode>,
    constants: ValueArray,
    lines: Vec<usize>,
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

    pub fn set_code(&mut self, byte: OpCode, index: usize) {
        self.code[index] = byte;
    }

    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn get_constant(&self, offset: usize) -> &Value {
        self.constants.get(offset)
    }

    pub fn has_constant(&self, constant: &Value) -> isize {
        for i in 0..self.constants.len() {
            if self.constants.get(i) == constant {
                return i as isize;
            }
        }

        return -1;
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.write_constant(constant);
        self.constants.len() - 1
    }

    pub fn get_line(&self, offset: usize) -> usize {
        self.lines[offset]
    }
}
