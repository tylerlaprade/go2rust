use crate::{zz_types::{Callback}};

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn apply(callback: crate::zz_types::Callback, x: Rc<RefCell<Option<i64>>>) -> i64 {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = callback.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone()) }
}