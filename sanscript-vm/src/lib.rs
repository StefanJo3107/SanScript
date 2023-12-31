use std::collections::HashMap;
use sanscript_common::chunk::{Chunk, OpCode};
use sanscript_common::debug::{disassemble_chunk, disassemble_instruction};
use sanscript_common::value::{Value, ValueArray};
use sanscript_frontend::compiler::Compiler;
use sanscript_frontend::scanner::Scanner;
use crate::InterpretResult::{InterpretCompileError, InterpretOK, InterpretRuntimeError};

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

#[derive(PartialEq, Copy, Clone)]
pub enum DebugLevel {
    None,
    TokenTableOnly,
    BytecodeOnly,
    Verbose,
}

const STACK_SIZE: usize = 256;


pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    debug_level: DebugLevel,
}

impl VM {
    pub fn new(debug_level: DebugLevel) -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
            globals: HashMap::new(),
            debug_level,
        }
    }

    // pub fn interpret(&mut self, name: &str) -> InterpretResult {
    //     self.run(name)
    // }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        if self.debug_level == DebugLevel::Verbose || self.debug_level == DebugLevel::TokenTableOnly {
            let mut scanner = Scanner::new(source.as_str());
            scanner.tokenize_source();
        }

        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source.as_str());

        if !compiler.compile(&mut chunk) {
            return InterpretCompileError;
        }

        self.chunk = chunk;
        self.ip = 0;

        let result = self.run();
        return result;
    }

    fn is_number_operands(&self) -> bool {
        return matches!(self.stack.last().unwrap_or_else(|| {panic!("Error reading last element of the stack!")}), Value::ValNumber(_)) && matches!(self.stack.get(self.stack.len() - 2).unwrap_or_else(||{panic!("Error reading second to last element of the stack!");}), Value::ValNumber(_));
    }

    fn is_string_operands(&self) -> bool {
        return matches!(self.stack.last().unwrap_or_else(|| {panic!("Error reading last element of the stack!")}), Value::ValString(_)) && matches!(self.stack.get(self.stack.len() - 2).unwrap_or_else(||{panic!("Error reading second to last element of the stack!");}), Value::ValString(_));
    }

    //most important function so far
    fn run(&mut self) -> InterpretResult {
        macro_rules! binary_op {
            (Value::ValString, +) => {
                if let Value::ValString(b) = self.stack.pop().unwrap() {
                    if let Value::ValString(a) = self.stack.pop().unwrap() {
                        self.stack.push(Value::ValString(format!("{}{}", a, b)));
                    }
                }
            };
            ($value_type: path,$op: tt) => {
                if !self.is_number_operands() {
                    self.runtime_error("Operands must be numbers.");
                    return InterpretRuntimeError;
                }
                if let Value::ValNumber(b) = self.stack.pop().unwrap() {
                    if let Value::ValNumber(a) = self.stack.pop().unwrap() {
                        self.stack.push($value_type(a $op b));
                    }
                }
            }
        }

        if self.debug_level == DebugLevel::Verbose || self.debug_level == DebugLevel::BytecodeOnly {
            //printing disassembler header
            println!("\x1B[4mOFFSET |  LINE  | {: <30}\x1B[0m", "OPCODE");
        }

        let mut print_offsets: Vec<usize> = vec![];
        print_offsets.push(0);
        let mut print_ip = self.ip;

        loop {
            let instruction: &OpCode = self.chunk.get_code(self.ip);

            if self.debug_level == DebugLevel::Verbose || self.debug_level == DebugLevel::BytecodeOnly {
                for ip in print_ip..self.ip + 1 {
                    if self.ip - ip >= 1 {
                        print!("\x1b[31m{:0>6} |", print_offsets.last().unwrap());
                    } else {
                        print!("\x1b[0m{:0>6} |", print_offsets.last().unwrap());
                    }
                    let off = disassemble_instruction(&self.chunk, ip, &mut print_offsets);
                    print_offsets.push(off);
                    //printing stack
                    for value in self.stack.iter() {
                        if self.ip - ip >= 1 {
                            print!("\x1b[31m[ ");
                            ValueArray::print_value(value);
                            print!("\x1b[31m ]");
                        } else {
                            print!("\x1b[0m[ ");
                            ValueArray::print_value(value);
                            print!("\x1b[0m ]");
                        }
                    }

                    if self.stack.len() > 0 {
                        println!("\x1b[0m");
                    }
                }
                if print_ip <= self.ip {
                    print_ip = self.ip + 1;
                }
            }

            match instruction
            {
                OpCode::OpReturn => {
                    return InterpretOK;
                }
                OpCode::OpPrint => {
                    ValueArray::print_value(&self.stack.pop().unwrap_or_else(|| { Value::ValString(String::from("")) }));
                    println!();
                }
                OpCode::OpPop => {
                    self.stack.pop();
                }
                OpCode::OpConstant(constant_addr) => {
                    let constant = self.chunk.get_constant(constant_addr.to_owned());
                    self.stack.push(constant.to_owned());
                }
                OpCode::OpDefineGlobal(global_addr) => {
                    let name_value = self.chunk.get_constant(global_addr.to_owned());
                    if let Value::ValString(name) = name_value {
                        self.globals.insert(name.to_owned(), self.stack.pop().unwrap_or_else(|| { panic!("Stack is empty, cannot define global variable") }));
                    }
                }
                OpCode::OpGetGlobal(global_addr) => {
                    let name_value = self.chunk.get_constant(global_addr.to_owned());
                    if let Value::ValString(name) = name_value {
                        if let Some(var_value) = self.globals.get(name) {
                            self.stack.push(var_value.to_owned());
                        } else {
                            self.runtime_error(format!("Undefined variable '{}'", name).as_str());
                            return InterpretRuntimeError;
                        }
                    }
                }
                OpCode::OpSetGlobal(global_addr) => {
                    let name_value = self.chunk.get_constant(global_addr.to_owned());
                    if let Value::ValString(name) = name_value {
                        if let Some(_) = self.globals.get(name) {
                            self.globals.insert(name.to_owned(), self.stack.pop().unwrap_or_else(|| { panic!("Stack is empty, cannot define global variable") }));
                        } else {
                            self.runtime_error(format!("Undefined variable '{}'", name).as_str());
                            return InterpretRuntimeError;
                        }
                    }
                }
                OpCode::OpGetLocal(local_addr) => {
                    let stack_val = self.stack.get(*local_addr).unwrap_or_else(|| { panic!("Stack is empty, cannot get local variable") });
                    self.stack.push(stack_val.clone());
                }
                OpCode::OpSetLocal(local_addr) => {
                    self.stack[*local_addr] = self.stack.last().unwrap_or_else(|| { panic!("Stack is empty, cannot set local variable") }).clone();
                }
                OpCode::OpNegate => {
                    if let Some(Value::ValNumber(number)) = self.stack.last() {
                        self.stack.push(Value::ValNumber(-*number));
                        //remove element that used to be last
                        self.stack.remove(self.stack.len() - 2);
                    } else {
                        self.runtime_error("Operand must be a number.");
                        return InterpretRuntimeError;
                    }
                }
                OpCode::OpAdd => {
                    if self.is_number_operands() {
                        binary_op!(Value::ValNumber, +);
                    }

                    if self.is_string_operands() {
                        binary_op!(Value::ValString, +);
                    }
                }
                OpCode::OpSubtract => {
                    binary_op!(Value::ValNumber, -);
                }
                OpCode::OpMultiply => {
                    binary_op!(Value::ValNumber, *);
                }
                OpCode::OpDivide => {
                    binary_op!(Value::ValNumber, /);
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::ValBool(true))
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::ValBool(false))
                }
                OpCode::OpNil => {
                    self.stack.push(Value::ValNil)
                }
                OpCode::OpNot => {
                    let value = self.stack.pop().unwrap_or_else(|| { panic!("Stack is empty."); });
                    self.stack.push(Value::ValBool(self.is_falsey(value)));
                }
                OpCode::OpEqual => {
                    let b = self.stack.pop().unwrap_or_else(|| { panic!("Stack is empty."); });
                    let a = self.stack.pop().unwrap_or_else(|| { panic!("Stack is empty."); });
                    self.stack.push(Value::ValBool(self.equals(a, b)));
                }
                OpCode::OpGreater => {
                    binary_op!(Value::ValBool, >);
                }
                OpCode::OpLess => {
                    binary_op!(Value::ValBool, <);
                }
                OpCode::OpJumpIfFalse(offset) => {
                    if self.is_falsey(self.stack.last().unwrap_or_else(|| { panic!("Stack is empty.") }).clone()) {
                        self.ip += offset;
                    }
                }
                OpCode::OpJumpIfTrue(offset) => {
                    if !self.is_falsey(self.stack.last().unwrap_or_else(|| { panic!("Stack is empty.") }).clone()) {
                        self.ip += offset;
                    }
                }
                OpCode::OpJump(offset) => {
                    self.ip += offset;
                }
                OpCode::OpLoop(offset) => {
                    self.ip -= offset;
                }
            };

            self.ip += 1;
        }
    }

    pub fn is_falsey(&self, value: Value) -> bool {
        return match value {
            Value::ValBool(boolean) => !boolean,
            Value::ValNumber(number) => number == 0.0,
            Value::ValNil => true,
            _ => false //TODO
        };
    }

    pub fn equals(&self, a: Value, b: Value) -> bool {
        return match (a, b) {
            (Value::ValNumber(num_a), Value::ValNumber(num_b)) => num_a == num_b,
            (Value::ValBool(bool_a), Value::ValBool(bool_b)) => bool_a == bool_b,
            (Value::ValNil, Value::ValNil) => true,
            (Value::ValString(string_a), Value::ValString(string_b)) => string_a == string_b,
            _ => false
        };
    }

    pub fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);

        eprintln!("[line {}] in script", self.chunk.get_line(self.ip - 1));
        self.stack = vec![];
    }
}