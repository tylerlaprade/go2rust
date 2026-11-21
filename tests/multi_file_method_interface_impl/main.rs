#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod counter_step;
mod counter_value;
mod iface;
use counter_step::*;
use counter_value::*;
use iface::*;

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn run(s: Rc<RefCell<Option<Box<dyn Stepper>>>>) -> i64 {
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow().as_ref().unwrap()).value()
}

fn main() {
    println!("{}", format!("{}", run(Rc::new(RefCell::new(Some(Box::new(CounterPtr(Rc::new(RefCell::new(Some(Counter { n: Rc::new(RefCell::new(Some(0))) }))).clone())) as Box<dyn Stepper>))))));
}