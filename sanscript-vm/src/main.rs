use san_vm::{DebugLevel, VM};

fn main() {
    // let source = "!(5 - 4 > 3 * 2 == !nil)";
    // let source = "print \"abc\"==\"abc\";
    // print 2+3;
    // print !(5 - 4 > 3 * 2 == !nil);
    // print 4/3;
    // print \" \";
    // let a = 3+2;";
    let source = "let prom = \"chrono\";
    prom = prom + \" trigger\";
    print prom;
    if (prom == \"chrono trigger\"){
        print \"Hello\";
        print \"World\";
    }
    {
        let a = 3;
        a = 4;
        print a;
        let b = 5;
        {
            let c = 25 + 4;
            print c;
        }
        print b;
        b = a;
        print b;
    }
    ";
    // let source = "print 3+(2*(4/3));";
    println!();
    println!("Source code:\n\t{}", source);
    println!();

    let mut vm = VM::new(DebugLevel::Verbose);
    vm.interpret(source.to_string());
}
