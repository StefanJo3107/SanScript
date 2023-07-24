use sanscript_common::{
    chunk::{Chunk, OpCode},
    debug::dissasemble_chunk,
};
pub mod runner;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::OpReturn);
    dissasemble_chunk(&chunk, "Test chunk");
}
