use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Path(pub Rc<RefCell<Option<String>>>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


pub fn text() -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("".to_string())))
}

pub fn object(p: Rc<RefCell<Option<Path>>>) -> Rc<RefCell<Option<String>>> {
    if (*p.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap().clone() == "".to_string() {
        return Rc::new(RefCell::new(Some("empty".to_string())));
    }
    Rc::new(RefCell::new(Some((*p.borrow().as_ref().unwrap()).to_string())))
}