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
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn matches() -> bool {
    let mut cause = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("cause".to_string())))));
    return (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = Rc::new(RefCell::new(go_errors_join(vec![{ let __err_handle = { let __err_source = cause.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))); let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = cause.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap());
}