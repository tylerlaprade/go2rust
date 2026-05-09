use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*args.borrow().as_ref().unwrap()).len() as i32)));
}