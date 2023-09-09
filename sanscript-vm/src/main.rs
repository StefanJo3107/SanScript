use san_vm::VM;
use sanscript_frontend::scanner::Scanner;

fn main() {
    let source = "!(5 - 4 > 3 * 2 == !nil)";
    println!();
    println!("Source code: {}", source);
    println!();
    let mut scanner = Scanner::new(source);
    scanner.tokenize_source();
    let mut vm = VM::new();
    vm.interpret(source.to_string());
}
