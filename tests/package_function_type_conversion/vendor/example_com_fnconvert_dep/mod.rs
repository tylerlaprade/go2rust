use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub type Exporter = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>;


pub fn set(e: Exporter) -> i32 {
    (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = e.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap())
}