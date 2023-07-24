use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
    println!("==== {} ====", name);

    let mut offset = 0;
    while offset < chunk.len() {
        offset = dissamble_instruction(chunk, offset);
    }
}

fn dissamble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:0>4} ", offset);

    let instruction = chunk.get(offset);
    match instruction {
        OpCode::OpReturn => return simple_instruction(instruction, offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

fn simple_instruction(opcode: &OpCode, offset: usize) -> usize {
    println!("{}", opcode);
    offset + 1
}
