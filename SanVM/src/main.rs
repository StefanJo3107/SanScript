use san_vm::VM;
use sanscript_frontend::scanner::Scanner;

fn main() {
    let source = "2+3";
    let mut vm = VM::new();
    vm.interpret(source.to_string());
    // let mut scanner = Scanner::new(source);
    // scanner.tokenize_source();
}
