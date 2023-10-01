use san_vm::VM;
use sanscript_frontend::scanner::Scanner;

fn main() {
    // let source = "!(5 - 4 > 3 * 2 == !nil)";
    let source = "print \"abc\"==\"abc\";
    print 2+3;
    print !(5 - 4 > 3 * 2 == !nil);
    print 4/3;
    print \" \";
    3+2;";
    // let source = "print 3+(2*(4/3));";
    println!();
    println!("Source code: {}", source);
    println!();
    let mut scanner = Scanner::new(source);
    scanner.tokenize_source();
    let mut vm = VM::new();
    vm.interpret(source.to_string());
}
