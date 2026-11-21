use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec, go_silent_catch_unwind};

use crate::{types::{splitError}};

use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
impl crate::types::splitError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("split:{}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl StdError for crate::types::splitError {}


pub fn __go_nil_recv_split_error_error(err: Rc<RefCell<Option<splitError>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("split:{}", (*(*err.borrow().as_ref().unwrap()).code.borrow().as_ref().unwrap())))))
}