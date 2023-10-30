use san_vm::{DebugLevel, VM};

fn main() {
    // let source = "!(5 - 4 > 3 * 2 == !nil)";
    // let source = "print \"abc\"==\"abc\";
    // print 2+3;
    // print !(5 - 4 > 3 * 2 == !nil);
    // print 4/3;
    // print \" \";
    // let a = 3+2;";
    let source = "let game = \"chrono\";
    game = game + \" trigger\";
    print game;";
    // let source = "print 3+(2*(4/3));";
    println!();
    println!("Source code: {}", source);
    println!();

    let mut vm = VM::new(DebugLevel::Verbose);
    vm.interpret(source.to_string());
}
