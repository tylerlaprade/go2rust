#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod flags;
use flags::*;

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn set_flag() -> u32 {
    let mut flags: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some(0)));
    { let __rhs = FLAG_SYNC_MARKERS as u32; let mut guard = flags.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    return (*flags.borrow().as_ref().unwrap());
}

fn main() {
    println!("{}", format!("{}", set_flag()));
}