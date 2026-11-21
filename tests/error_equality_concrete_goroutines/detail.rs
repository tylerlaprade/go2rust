use crate::{
    GoChannel,
    GoSlice,
    GoSliceAppendBoxed,
    GoSliceFromSlice,
    GoSliceHeader,
    GoSliceView,
    GoWrapError,
    Vec,
    go_error_chain_visit,
    go_error_wrap,
    go_errors_is,
    go_errors_join,
    go_errors_unwrap,
    go_silent_catch_unwind,
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// This field type is emitted before the error type that requires PartialEq.
#[derive(Debug, Clone)]
pub struct nestedCode {
    pub code: Arc<Mutex<Option<String>>>,
}

impl nestedCode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for nestedCode {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nestedCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.code.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for nestedCode {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.code.lock().unwrap().as_ref().cloned() }; let __right = { other.code.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}
