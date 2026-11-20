#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod aaa;
mod bbb;
use aaa::*;
use bbb::*;

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn main() {
    __go_init_all();

    println!("{} {} {}", format!("{}", { let __v = (*A.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*B.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*C.borrow().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    aaa::__go_zero_globals();
    bbb::__go_zero_globals();
    aaa::__go_init_order_0();
    bbb::__go_init_order_1();
    aaa::__go_init_order_2();
}
