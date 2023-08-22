use san_vm::VM;
use sanscript_frontend::scanner::Scanner;

fn main() {
    let source = "let a = 4;
    if a > 3 {
        return;
    }";
    let mut scanner = Scanner::new(source);
    scanner.tokenize_source();
}
