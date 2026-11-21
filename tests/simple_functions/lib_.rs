use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn get_hello() -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("Hello".to_string())))
}

pub fn get_world() -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("World".to_string())))
}

pub fn get_magic_number() -> i64 {
    42
}