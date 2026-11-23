#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod support;
use support::*;

use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{}", format!("{}", count(Rc::new(RefCell::new(Some(vec![Box::new("label".to_string()) as Box<dyn Any>, Box::new(7i64) as Box<dyn Any>, Box::new(true) as Box<dyn Any>]))))));
}