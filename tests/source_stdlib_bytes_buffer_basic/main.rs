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
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut buf: Rc<RefCell<Option<bytes::buffer::Buffer>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", (*(*buf.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap()).clone() == "".to_string()));
    (*buf.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("go".to_string()))));
    (*buf.borrow_mut().as_mut().unwrap()).write_byte(Rc::new(RefCell::new(Some(('2' as u8) as u8))));
    (*buf.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("rust".to_string()))));
    println!("{} {}", format!("{}", (*buf.borrow().as_ref().unwrap()).len()), format!("{}", (*(*buf.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap())));
}