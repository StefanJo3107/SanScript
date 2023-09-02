use std::cell::RefCell;
use std::rc::Rc;
use crate::scanner::Scanner;

pub mod runner;
pub mod compiler;
pub mod scanner;
pub mod token;
pub mod parser;

type ScannerRef<'a> = Rc<RefCell<Scanner<'a>>>;
