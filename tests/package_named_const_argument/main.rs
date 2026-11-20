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
    ::example_com_package_named_const_argument_helper::__go_init_all();

    let mut v = example_com_package_named_const_argument_helper::new_var();
    example_com_package_named_const_argument_helper::set_kind(v.clone(), Rc::new(RefCell::new(Some(example_com_package_named_const_argument_helper::r#mod::Kind(Rc::new(RefCell::new(Some(example_com_package_named_const_argument_helper::PACKAGE_VAR as u8))))))));
    println!("{}", format!("{}", "ok".to_string()));
}