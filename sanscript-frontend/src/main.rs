use sanscript_common::{
    chunk::{Chunk, OpCode},
};
use sanscript_common::value::Value;

fn main() {
    let mut chunk = Chunk::new();

    let mut const_offset = chunk.add_constant(Value::ValNumber(4.2));
    chunk.write_chunk(OpCode::OpConstant(const_offset), 123);
    const_offset = chunk.add_constant(Value::ValNumber(2.4));
    chunk.write_chunk(OpCode::OpConstant(const_offset), 124);
    chunk.write_chunk(OpCode::OpNegate, 124);
    chunk.write_chunk(OpCode::OpSubtract, 125);
    chunk.write_chunk(OpCode::OpReturn, 126);
    // let mut vm = VM::new(&chunk);
    // vm.interpret("TEST CHUNK");
    // disassemble_chunk(&chunk, "TEST CHUNK");
}
