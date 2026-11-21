use crate::{
    GoSlice,
    GoSliceAppendBoxed,
    GoSliceFromSlice,
    GoSliceHeader,
    GoWrapError,
    Vec,
    go_error_chain,
    go_error_wrap,
    go_errors_is,
    go_errors_join,
    go_errors_unwrap,
};

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Kept in an earlier package file than nestedError: package-wide error
/// equality analysis must still give this field type Rust equality support.
#[derive(Debug, Clone, PartialEq)]
pub struct nestedCode {
    pub code: Rc<RefCell<Option<String>>>,
}

impl nestedCode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for nestedCode {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nestedCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.code.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
