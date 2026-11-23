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
    ::example_com_selectorzero_model::__go_init_all();

    let mut idx: Rc<RefCell<Option<example_com_selectorzero_model::r#mod::Index>>> = Rc::new(RefCell::new(Some(example_com_selectorzero_model::r#mod::Index(Rc::new(RefCell::new(Some(0)))))));
    println!("{}", format!("{}", { let __v = (*idx.borrow().as_ref().unwrap()).clone(); __v }));
}