use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use crate::{zz_types::{Callback}};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn apply(callback: crate::zz_types::Callback, x: Rc<RefCell<Option<i64>>>) -> i64 {
    { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = callback.borrow().as_ref().unwrap().clone(); (__f)(x.clone()) }
}