#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod codes;
use codes::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Writer {
}

impl Writer {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Writer {
    pub fn code(&self, c: Rc<RefCell<Option<Box<dyn Code>>>>) -> i64 {
        (*c.borrow().as_ref().unwrap()).value()
    }
}

fn main() {
    println!("{}", format!("{}", (Writer {  }).code(Rc::new(RefCell::new(Some(Box::new(CodeVal(Rc::new(RefCell::new(Some(VAL_BOOL as i64))))) as Box<dyn Code>))))));
    println!("{}", format!("{}", (Writer {  }).code(Rc::new(RefCell::new(Some(Box::new(CodeVal(Rc::new(RefCell::new(Some(VAL_STRING as i64))))) as Box<dyn Code>))))));
}