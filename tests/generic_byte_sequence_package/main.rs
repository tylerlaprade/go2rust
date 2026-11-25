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
    ::example_com_seqpkg_seq::__go_init_all();

    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::trim::<String>(Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*example_com_seqpkg_seq::trim::<Vec<u8>>(Rc::new(RefCell::new(Some(("hello".to_string()).as_bytes().to_go_slice()))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::mid::<String>(Rc::new(RefCell::new(Some("abcdef".to_string()))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*example_com_seqpkg_seq::mid::<Vec<u8>>(Rc::new(RefCell::new(Some(("abcdef".to_string()).as_bytes().to_go_slice()))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", example_com_seqpkg_seq::digits::<String>(Rc::new(RefCell::new(Some("a1b22c".to_string()))))), format!("{}", example_com_seqpkg_seq::digits::<Vec<u8>>(Rc::new(RefCell::new(Some(("9x9".to_string()).as_bytes().to_go_slice()))))));
    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::prefix::<String>(Rc::new(RefCell::new(Some("xyz".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_seqpkg_seq::prefix::<Vec<u8>>(Rc::new(RefCell::new(Some(("pq!".to_string()).as_bytes().to_go_slice())))).borrow().as_ref().unwrap())));
    let (mut n, mut rest) = example_com_seqpkg_seq::atoi::<String>(Rc::new(RefCell::new(Some("123abc".to_string()))));
    let (mut m, mut restb) = example_com_seqpkg_seq::atoi::<Vec<u8>>(Rc::new(RefCell::new(Some(("42!".to_string()).as_bytes().to_go_slice()))));
    let mut restb = { let __slice_header = GoSliceHeader::go_slice_header(&restb); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {} {} {}", format!("{}", n), format!("{}", { let __v = (*rest.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", m), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*restb.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    let mut b = Rc::new(RefCell::new(Some(("shared".to_string()).as_bytes().to_go_slice())));
    let mut t = example_com_seqpkg_seq::trim::<Vec<u8>>(b.clone(), Rc::new(RefCell::new(Some(1))));
    let mut t = { let __slice_header = GoSliceHeader::go_slice_header(&t); Rc::new(RefCell::new(__slice_header)) };
    { let __idx = (0) as usize; let __rhs = ('H' as u8) as u8; (*t.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*b.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*t.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
}