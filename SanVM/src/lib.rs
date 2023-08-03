use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::value::{Value, ValueArray};
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

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }

    //most important function so far
    fn run(&mut self) -> InterpretResult{
        loop {
            let instruction:&OpCode = self.chunk.get_code(self.ip);

            match  instruction
            {
                OpCode::OpReturn => return InterpretOK,
                OpCode::OpConstant(constant_addr)=> {
                    let constant = self.chunk.get_constant(constant_addr.to_owned());
                    ValueArray::print_value(constant);
                    println!();
                    break;
                }
            };

            self.ip+=1;
        }

        InterpretOK
    }
}