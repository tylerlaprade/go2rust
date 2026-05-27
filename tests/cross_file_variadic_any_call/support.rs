use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i32 {

    return (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32;
}