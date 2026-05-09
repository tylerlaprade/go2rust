use crate::zz_types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn apply(callback: Callback, x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return { let __f_guard = callback.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(x.clone()) };
}