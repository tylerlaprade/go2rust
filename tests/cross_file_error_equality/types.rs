use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

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
#[derive(Debug, Clone)]
pub struct splitError {
    pub code: Rc<RefCell<Option<i64>>>,
}

impl splitError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for splitError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for splitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl splitError {
}
#[derive(Debug, Clone)]
pub struct __go_error_ptr_splitError(pub Rc<RefCell<Option<splitError>>>);

impl std::fmt::Display for __go_error_ptr_splitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_previous_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let __go_error_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::methods::__go_nil_recv_split_error_error(self.0.clone())));
            std::panic::set_hook(__go_previous_panic_hook);
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_splitError {}
