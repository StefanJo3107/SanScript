use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::debug::{disassemble_chunk, disassemble_instruction};
use sanscript_common::value::ValueArray;
use crate::InterpretResult::InterpretOK;

pub enum InterpretResult{
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError
}

pub struct VM<'a>{
    chunk: &'a Chunk,
    ip: usize
}

impl<'a> VM<'a>{
    pub fn new(chunk: &'a Chunk) -> VM{
        VM{
            chunk,
            ip: 0
        }
    }

    pub fn interpret(&mut self, name: &str) -> InterpretResult {
        self.run(name)
    }

    //most important function so far
    fn run(&mut self, name: &str) -> InterpretResult{
        //printing disassembler header
        println!("\x1B[4mCODE |  LINE  | {: <30}\x1B[0m", name);
        let mut print_offset = 0;

        loop {
            let instruction:&OpCode = self.chunk.get_code(self.ip);

            print!("{:0>4} |", print_offset);
            print_offset = disassemble_instruction(self.chunk, self.ip, print_offset);

            match  instruction
            {
                OpCode::OpReturn => return InterpretOK,
                OpCode::OpConstant(constant_addr)=> {
                    let constant = self.chunk.get_constant(constant_addr.to_owned());
                    // ValueArray::print_value(constant);
                    // println!();
                }
            };

            self.ip+=1;
        }

        InterpretOK
    }
}