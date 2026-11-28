#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod helpers;
use helpers::*;

use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    go_install_panic_exit();
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<AnonymousStruct1, Rc<RefCell<Option<bool>>>>::from([]))));
    { let __map_key = { let __v = origin(); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }; let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = AnonymousStruct1 { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))) }; let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", ({ let __len = (*seen.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map_holder = seen.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&{ let __v = origin(); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) }));
}