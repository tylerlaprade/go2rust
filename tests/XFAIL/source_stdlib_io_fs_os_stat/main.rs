#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    ::io_fs::__go_init_all();

    let (mut info, mut err) = os::stat(".".to_string());
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        panic!("{}", (*err.borrow().as_ref().unwrap()));
    }
    println!("{}", format!("{}", (*io_fs::r#mod::FileMode::r#type(&(*(*info.borrow().as_ref().unwrap()).mode().borrow().as_ref().unwrap())).borrow().as_ref().unwrap()).clone() == io_fs::r#mod::FileMode(Rc::new(RefCell::new(Some(io_fs::MODE_DIR as u32))))));
}