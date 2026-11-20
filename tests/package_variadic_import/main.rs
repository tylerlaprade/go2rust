#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn main() {
    ::example_com_packagevariadic_label::__go_init_all();

    println!("{}", format!("{}", (*example_com_packagevariadic_label::count(Rc::new(RefCell::new(Some("empty".to_string()))), Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_packagevariadic_label::count(Rc::new(RefCell::new(Some("full".to_string()))), Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])))).borrow().as_ref().unwrap())));
}