#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod root;
mod sibling;
mod r#use;
use root::*;
use sibling::*;
use r#use::*;

use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
impl rootError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        if false {
        return Rc::new(RefCell::new(Some("root-nil".to_string())));
    }
        Rc::new(RefCell::new(Some("root".to_string())))
    }
}

impl StdError for rootError {}


fn main() {
    println!("{}", format!("{}", (*call_root(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*call_sibling(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap())));
}

pub fn __go_nil_recv_root_error_error(receiver: Rc<RefCell<Option<rootError>>>) -> Rc<RefCell<Option<String>>> {
    if { let __nil_result = (*receiver.borrow()).is_none(); __nil_result } {
        return Rc::new(RefCell::new(Some("root-nil".to_string())));
    }
    Rc::new(RefCell::new(Some("root".to_string())))
}