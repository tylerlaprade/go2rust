use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn panicf(format: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
    panic!("{}", (*format.borrow().as_ref().unwrap()).clone());
}

fn main() {
    println!("{}", format!("{}", "ok".to_string()));
}