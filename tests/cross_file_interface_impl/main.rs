#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod circle;
mod shape;
use circle::*;
use shape::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    go_install_panic_exit();
    let mut shapes = Rc::new(RefCell::new(Some(vec![new_circle(Rc::new(RefCell::new(Some(2)))).clone(), new_circle(Rc::new(RefCell::new(Some(3)))).clone()])));
    println!("{}", format!("{}", total_area(shapes.clone())));
}