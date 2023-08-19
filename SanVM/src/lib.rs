use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::debug::disassemble_instruction;
use sanscript_common::value::{Value, ValueArray};
use sanscript_frontend::compiler;
use crate::InterpretResult::InterpretOK;

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_SIZE: usize = 256;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: vec![],
        }
    }

    // pub fn interpret(&mut self, name: &str) -> InterpretResult {
    //     self.run(name)
    // }

    pub fn interpret(source: String) -> InterpretResult {
        compiler::compile(source);
        InterpretOK
    }

    //most important function so far
    fn run(&mut self, name: &str) -> InterpretResult {
        //printing disassembler header
        println!("\x1B[4mCODE |  LINE  | {: <30}\x1B[0m", name);
        let mut print_offset = 0;

        loop {
            let instruction: &OpCode = self.chunk.get_code(self.ip);

            print!("{:0>4} |", print_offset);
            print_offset = disassemble_instruction(self.chunk, self.ip, print_offset);

            //printing stack
            for value in self.stack.iter() {
                print!("[ ");
                ValueArray::print_value(*value);
                print!(" ]");
            }
            println!();

            match instruction
            {
                OpCode::OpReturn => {
                    ValueArray::print_value(self.stack.pop().unwrap());
                    return InterpretOK;
                },
                OpCode::OpConstant(constant_addr) => {
                    let constant = self.chunk.get_constant(constant_addr.to_owned());
                    self.stack.push(*constant);
                },
                OpCode::OpNegate => {
                    let top = self.stack.pop().unwrap();
                    self.stack.push(-top);
                },
                OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                OpCode::OpSubtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                },
                OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                },
                OpCode::OpDivide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                },
            };

            self.ip += 1;
        }
    }
}