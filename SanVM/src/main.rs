use san_vm::VM;

fn main() {
    VM::interpret("let a = 0;".to_string());
}
