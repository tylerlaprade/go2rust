#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod aa_use;
mod zz_types;
use aa_use::*;
use zz_types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    go_install_panic_exit();
    println!("{}", format!("{}", apply(Rc::new(RefCell::new(Some(Rc::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { double(__arg0) }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>))), Rc::new(RefCell::new(Some(21))))));
}

pub fn double(x: Rc<RefCell<Option<i64>>>) -> i64 {
    (*x.borrow().as_ref().unwrap()) * 2
}