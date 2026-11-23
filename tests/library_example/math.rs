use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Add adds two numbers
pub fn add(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap())
}

/// Multiply multiplies two numbers
pub fn multiply(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> i64 {
    (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap())
}