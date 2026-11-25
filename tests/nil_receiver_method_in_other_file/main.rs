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
    ::example_com_nilrecv_list::__go_init_all();

    let mut empty: Rc<RefCell<Option<example_com_nilrecv_list::r#mod::List>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", example_com_nilrecv_list::__go_nil_recv_list_len(empty.clone())), format!("{}", example_com_nilrecv_list::sum(empty.clone())));
    let mut l = example_com_nilrecv_list::push(example_com_nilrecv_list::push(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(2)))), Rc::new(RefCell::new(Some(3))));
    println!("{} {}", format!("{}", example_com_nilrecv_list::__go_nil_recv_list_len(l.clone())), format!("{}", example_com_nilrecv_list::sum(l.clone())));
}