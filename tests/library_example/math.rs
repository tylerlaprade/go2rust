use std::cell::{RefCell};
use std::rc::{Rc};

/// Add adds two numbers
pub fn add(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap())
}

/// Multiply multiplies two numbers
pub fn multiply(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap())
}