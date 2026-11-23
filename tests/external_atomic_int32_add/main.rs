#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    ::example_com_atomicdep_dep::__go_init_all();

    let mut counter = example_com_atomicdep_dep::new_counter();
    println!("{}", format!("{}", (*counter.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(1 as i32))))));
    println!("{}", format!("{}", (*counter.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(-1 as i32))))));
}