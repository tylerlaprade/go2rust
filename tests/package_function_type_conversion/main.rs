#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

pub type Exporter = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>;


pub fn r#use(e: Exporter) -> i64 {
    example_com_fnconvert_dep::set(e.clone())
}

fn main() {
    ::example_com_fnconvert_dep::__go_init_all();

    println!("{}", format!("{}", r#use(Rc::new(RefCell::new(Some(Box::new(move |v: Rc<RefCell<Option<i64>>>| -> i64 {
        (*v.borrow().as_ref().unwrap()) * 2
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))))));
}