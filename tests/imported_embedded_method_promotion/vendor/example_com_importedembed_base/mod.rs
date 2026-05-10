use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Decoder {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Decoder {
    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*n.borrow().as_ref().unwrap())); };
    }

    pub fn label(&self, prefix: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}:{}", { let __v = (*prefix.borrow().as_ref().unwrap()).clone(); __v }, (*self.value.borrow().as_ref().unwrap())))));
    }

    pub fn snapshot(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}