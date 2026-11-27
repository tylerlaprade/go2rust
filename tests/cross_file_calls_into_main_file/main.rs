#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod other;
use other::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    go_install_panic_exit();
    other();
    let (mut __go_mv_2_0, mut __go_mv_2_1) = divmod(Rc::new(RefCell::new(Some(9))), Rc::new(RefCell::new(Some(4))));
    println!("{} {}", format!("{}", __go_mv_2_0), format!("{}", __go_mv_2_1));
}

pub fn divmod(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()))
}