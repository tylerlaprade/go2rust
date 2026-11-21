use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Repeat repeats a string n times
pub fn repeat(s: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>> {
    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&(*s.borrow().as_ref().unwrap())); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __owned = result.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}