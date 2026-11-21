use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec, go_silent_catch_unwind};

use crate::{sibling::{siblingError}};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn call_sibling(receiver: Rc<RefCell<Option<siblingError>>>) -> Rc<RefCell<Option<String>>> {
    crate::sibling::__go_nil_recv_sibling_error_error(receiver.clone())
}