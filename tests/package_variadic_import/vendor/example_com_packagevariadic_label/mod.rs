use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(prefix: Rc<RefCell<Option<String>>>, labels: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some(format!("{}:{}", { let __v = (*prefix.borrow().as_ref().unwrap()).clone(); __v }, (*labels.borrow().as_ref().unwrap()).len()))));
}