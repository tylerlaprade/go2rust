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
        Self {  }
    }
}

impl std::fmt::Display for Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Writer {
    pub fn code(&self, c: Rc<RefCell<Option<Box<dyn Code>>>>) -> i32 {
        (*c.borrow().as_ref().unwrap()).value()
    }
}

fn main() {
    println!("{}", format!("{}", (Writer {  }).code(Rc::new(RefCell::new(Some(Box::new(VAL_BOOL) as Box<dyn Code>))))));
    println!("{}", format!("{}", (Writer {  }).code(Rc::new(RefCell::new(Some(Box::new(VAL_STRING) as Box<dyn Code>))))));
}