use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

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
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = callback.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone()) }
}