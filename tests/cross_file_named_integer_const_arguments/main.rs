#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn take_reloc(k: Rc<RefCell<Option<RelocKind>>>) -> i64 {
    (*Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())
}

fn main() {
    let mut e: Rc<RefCell<Option<Encoder>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", take_reloc(Rc::new(RefCell::new(Some(RelocKind(Rc::new(RefCell::new(Some(RELOC_META as i32))))))))));
    println!("{}", format!("{}", (*e.borrow().as_ref().unwrap()).sync(Rc::new(RefCell::new(Some(SyncMarker(Rc::new(RefCell::new(Some(SYNC_BOOL as i64))))))))));
    println!("{}", format!("{}", (*e.borrow().as_ref().unwrap()).call_sync()));
}