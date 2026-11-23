use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec, go_silent_catch_unwind};

use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Clone, Default)]
pub struct siblingError {
}

impl siblingError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for siblingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl siblingError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        if false {
        return Rc::new(RefCell::new(Some("sibling-nil".to_string())));
    }
        Rc::new(RefCell::new(Some("sibling".to_string())))
    }
}

impl StdError for siblingError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_siblingError(pub Rc<RefCell<Option<siblingError>>>);

impl std::fmt::Display for __go_error_ptr_siblingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_sibling_error_error(Rc::new(RefCell::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_siblingError {}


pub fn __go_nil_recv_sibling_error_error(receiver: Rc<RefCell<Option<siblingError>>>) -> Rc<RefCell<Option<String>>> {
    if { let __nil_result = (*receiver.borrow()).is_none(); __nil_result } {
        return Rc::new(RefCell::new(Some("sibling-nil".to_string())));
    }
    Rc::new(RefCell::new(Some("sibling".to_string())))
}