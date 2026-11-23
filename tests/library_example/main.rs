#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod math;
mod string;
use math::*;
use string::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Test math functions
    let mut sum = add(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3))));
    print!("5 + 3 = {}\n", sum);

    let mut product = multiply(Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(7))));
    print!("4 * 7 = {}\n", product);

        // Test string function
    let mut repeated = repeat(Rc::new(RefCell::new(Some("Go".to_string()))), Rc::new(RefCell::new(Some(3))));
    print!("Repeat(\"Go\", 3) = {}\n", { let __v = (*repeated.borrow().as_ref().unwrap()).clone(); __v });
}