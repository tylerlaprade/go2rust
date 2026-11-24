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
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::io_fs::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut mode = Rc::new(RefCell::new(Some(io_fs::r#mod::FileMode(Rc::new(RefCell::new(Some(((io_fs::MODE_DIR as u32 | io_fs::MODE_SETUID as u32) as u32 | 0o640 as u32) as u32)))))));
    println!("{}", format!("{}", (*io_fs::r#mod::FileMode::r#type(&(*mode.borrow().as_ref().unwrap())).borrow().as_ref().unwrap()).clone() == io_fs::r#mod::FileMode(Rc::new(RefCell::new(Some(io_fs::MODE_DIR as u32))))));
    println!("{}", format!("{}", (*io_fs::r#mod::FileMode::string(&(*mode.borrow().as_ref().unwrap())).borrow().as_ref().unwrap())));
}