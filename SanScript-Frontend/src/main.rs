use sanscript_common::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
};
pub mod runner;

fn main() {
    let mut chunk = Chunk::new();

    let mut const_offset = chunk.add_constant(4.2);
    chunk.write_chunk(OpCode::OpConstant(const_offset), 123);
    const_offset = chunk.add_constant(2.4);
    chunk.write_chunk(OpCode::OpConstant(const_offset), 124);
    chunk.write_chunk(OpCode::OpReturn, 125);
    disassemble_chunk(&chunk, "TEST CHUNK");
}
