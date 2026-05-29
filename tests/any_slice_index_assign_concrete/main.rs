use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn set(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i32 {
    if ((*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) > (0 as i32) {
        (*args.borrow_mut().as_mut().unwrap())[(0) as usize] = Box::new(42) as Box<dyn Any>;
    }
    (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

fn main() {
    eprintln!("{}", format!("{}", set(Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("a".to_string()) as Box<dyn Any>, Box::new(3) as Box<dyn Any>]))))));
}