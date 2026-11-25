use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i64 {
    ({ let __len = (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64
}