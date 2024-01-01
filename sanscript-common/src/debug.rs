use std::mem::{size_of, size_of_val};
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

    let mut print_offsets:Vec<usize> = vec![];
    print_offsets.push(0);

    while offset < chunk.len() {
        //printing offset of an opcode
        print!("{:0>6} |", print_offsets.last().unwrap());

        disassemble_instruction(chunk, offset, &mut print_offsets);
        offset += 1;
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize, print_offsets: &mut Vec<usize>) -> usize {
    //printing line number in a source code
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("  -||-  |");
    } else {
        print!(" {: ^6} |", chunk.get_line(offset));
    }

    let instruction = chunk.get_code(offset);
    if matches_simple_instruction(instruction) {
        return simple_instruction(instruction, *print_offsets.last().unwrap());
    } else if matches_constant_instruction(instruction).0 {
        return constant_instruction(instruction, chunk.get_constant(matches_constant_instruction(instruction).1), *print_offsets.last().unwrap());
    } else if matches_byte_instruction(instruction).0 {
        return byte_instruction(instruction, matches_byte_instruction(instruction).1, *print_offsets.last().unwrap());
    } else if matches_jump_instruction(instruction).0
    {
        let jump_off = get_instruction_address(chunk, offset, *print_offsets.last().unwrap(), matches_jump_instruction(instruction).1);
        return jump_instruction(instruction, jump_off, *print_offsets.last().unwrap());
    } else if matches_loop_instruction(instruction).0
    {
        let last_index = print_offsets.len() - 1;
        let loop_off = print_offsets[last_index - matches_loop_instruction(instruction).1];
        return jump_instruction(instruction, loop_off, *print_offsets.last().unwrap());
    }

    panic!("Unknown opcode, terminating...");
}

fn matches_simple_instruction(opcode: &OpCode) -> bool {
    match opcode {
        OpCode::OpReturn | OpCode::OpNegate | OpCode::OpAdd | OpCode::OpSubtract | OpCode::OpMultiply
        | OpCode::OpDivide | OpCode::OpTrue | OpCode::OpFalse | OpCode::OpNil | OpCode::OpNot
        | OpCode::OpEqual | OpCode::OpGreater | OpCode::OpLess | OpCode::OpPrint | OpCode::OpPop => true,
        _ => false
    }
}

fn matches_constant_instruction(opcode: &OpCode) -> (bool, usize) {
    match opcode {
        OpCode::OpConstant(value) | OpCode::OpDefineGlobal(value) | OpCode::OpGetGlobal(value)
        | OpCode::OpSetGlobal(value) => (true, *value),
        _ => (false, 0)
    }
}

fn matches_byte_instruction(opcode: &OpCode) -> (bool, usize) {
    match opcode
    {
        OpCode::OpGetLocal(value) | OpCode::OpSetLocal(value) => (true, *value),
        _ => (false, 0)
    }
}

fn matches_jump_instruction(opcode: &OpCode) -> (bool, usize) {
    match opcode
    {
        OpCode::OpJumpIfFalse(value) | OpCode::OpJumpIfTrue(value) | OpCode::OpJump(value) => (true, *value),
        _ => (false, 0)
    }
}

fn matches_loop_instruction(opcode: &OpCode) -> (bool, usize) {
    match opcode
    {
        OpCode::OpLoop(value) => (true, *value),
        _ => (false, 0)
    }
}

fn get_instruction_address(chunk: &Chunk, offset: usize, print_offset: usize, jump_offset: usize) -> usize {
    let mut curr_offset = offset;
    let mut curr_print_offset = print_offset;
    while curr_offset < offset + jump_offset {
        let instruction = chunk.get_code(curr_offset);
        curr_print_offset += if matches_simple_instruction(instruction) { 1 } else { 8 };
        curr_offset += 1;
    }

    curr_print_offset
}

fn simple_instruction(opcode: &OpCode, offset: usize) -> usize {
    println!(" {}", opcode);
    offset + 1
}

fn constant_instruction(opcode: &OpCode, value: &Value, offset: usize) -> usize {
    match opcode {
        OpCode::OpConstant(index) | OpCode::OpDefineGlobal(index) | OpCode::OpGetGlobal(index) | OpCode::OpSetGlobal(index) => {
            print!(" {:<16} {:>4} '", opcode, index);
            //printing operand value
            ValueArray::print_value(value);
            println!("'");
            offset + size_of::<usize>()
        }
        _ => {
            eprintln!("Invalid opcode passed as a constant instruction!");
            exit(1);
        }
    }
}

fn byte_instruction(opcode: &OpCode, value: usize, offset: usize) -> usize {
    println!(" {:<16} {:>4}", opcode, value);
    offset + size_of::<usize>()
}

fn jump_instruction(opcode: &OpCode, jump_address: usize, offset: usize) -> usize {
    println!(" {:<16}  {:>4} -> {}", opcode, offset, jump_address);
    offset + size_of::<usize>()
}