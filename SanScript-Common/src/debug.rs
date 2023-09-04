use std::mem::size_of;
use std::process::exit;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::{Value, ValueArray};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    //printing disassembler header
    println!("\x1B[4mOFFSET |  LINE  | {: <30}\x1B[0m", name);

    //offset of a chunk code array
    let mut offset = 0;
    //offset that represents actual size of each instruction (used for printing only)
    let mut print_offset = 0;

    while offset < chunk.len() {
        //printing offset of an opcode
        print!("{:0>6} |", print_offset);

        print_offset = disassemble_instruction(chunk, offset, print_offset);
        offset += 1;
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize, print_offset: usize) -> usize {
    //printing line number in a source code
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("  -||-  |");
    } else {
        print!(" {: ^6} |", chunk.get_line(offset));
    }

    let instruction = chunk.get_code(offset);
    match instruction {
        OpCode::OpReturn | OpCode::OpNegate | OpCode::OpAdd | OpCode::OpSubtract | OpCode::OpMultiply
        | OpCode::OpDivide | OpCode::OpTrue | OpCode::OpFalse | OpCode::OpNil | OpCode::OpNot
        | OpCode::OpEqual | OpCode::OpGreater | OpCode::OpLess => simple_instruction(instruction, print_offset),
        OpCode::OpConstant(value) => constant_instruction(instruction, chunk.get_constant(value.to_owned()), print_offset),
    }
}

fn simple_instruction(opcode: &OpCode, offset: usize) -> usize {
    println!(" {}", opcode);
    offset + 1
}

fn constant_instruction(opcode: &OpCode, value: &Value, offset: usize) -> usize {
    if let OpCode::OpConstant(index) = opcode {
        //printing instruction name with its operand index
        print!(" {:<16} {:>4} '", opcode, index);
        //printing operand value
        ValueArray::print_value(*value);
        println!("'");
        offset + size_of::<usize>() + 1
    } else {
        eprintln!("Invalid opcode passed as a constant instruction!");
        exit(1);
    }
}
