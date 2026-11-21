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
    ::unicode_utf8::__go_init_all();

    let mut buf: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(Some(Vec::from(std::array::from_fn::<_, 10, _>(|_| 0)))));
    let mut n = unicode_utf8::encode_rune(Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = (0) as usize; let __high = __seq.len(); let __max = __seq.capacity(); __seq.reslice3(__low, __high, __max) }))), Rc::new(RefCell::new(Some(('\u{4e16}' as i32)))));
    let mut b1 = Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = (0) as usize; let __high = (n) as usize; let __max = __seq.capacity(); __seq.reslice3(__low, __high, __max) })));
    println!("{} {} {} {} {}", format!("{}", n), format!("{}", b1.borrow().as_ref().unwrap().get_cloned((0) as usize)), format!("{}", b1.borrow().as_ref().unwrap().get_cloned((1) as usize)), format!("{}", b1.borrow().as_ref().unwrap().get_cloned((2) as usize)), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*b1.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
}