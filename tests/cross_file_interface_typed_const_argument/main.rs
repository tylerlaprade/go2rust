mod codes;
use codes::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Writer {
}

impl std::fmt::Display for Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Writer {
    pub fn code(&self, c: &dyn Code) -> Rc<RefCell<Option<i32>>> {
        return c.value();
    }
}

fn main() {
    println!("{}", (*(Writer {  }).code(&CodeVal(Rc::new(RefCell::new(Some(VAL_BOOL as i32))))).borrow().as_ref().unwrap()));
    println!("{}", (*(Writer {  }).code(&CodeVal(Rc::new(RefCell::new(Some(VAL_STRING as i32))))).borrow().as_ref().unwrap()));
}