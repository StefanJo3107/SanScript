use san_vm::VM;

fn main() {
    VM::interpret("let a = 0;
    a = 3;
    for i in a{
    print \"hello\"
    }".to_string());
}
