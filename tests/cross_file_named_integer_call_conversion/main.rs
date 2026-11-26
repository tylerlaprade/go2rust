#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn raw_marker() -> u64 {
    5
}

fn main() {
    go_install_panic_exit();
    let mut marker = Rc::new(RefCell::new(Some(Marker(Rc::new(RefCell::new(Some(raw_marker() as i64)))))));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some((*(*marker.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())));
}