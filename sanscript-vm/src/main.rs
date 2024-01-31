use san_vm::{DebugLevel, VM};

fn main() {
    // let source = "!(5 - 4 > 3 * 2 == !nil)";
    // let source = "print \"abc\"==\"abc\";
    // print 2+3;
    // print !(5 - 4 > 3 * 2 == !nil);
    // print 4/3;
    // print \" \";
    // let a = 3+2;";
    let source = "
    fn funkcija(){
        print \"Hello\";
    }
    print funkcija;
    let prom = \"chrono\";
    let i = 0;

    while(i<5)
    {
        print i;
        i = i + 1;
    }
   
    for(let i = 5;i<10;i=i+1){
        print i;
    }
    
    prom = prom + \" trigger\";
    print prom;
    
    if (prom == \"chroono trigger\"){
        print \"Hello\";
        print \"World\";
    } else if (prom == \"chrono trigger\" and 3<2){
        print \"Za warudo\";
    } else {
        print \"Please stand up\";
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

    let mut vm = VM::new(DebugLevel::None);
    vm.interpret(source.to_string());
}
