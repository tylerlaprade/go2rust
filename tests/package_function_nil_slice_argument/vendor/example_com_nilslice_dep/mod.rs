use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    if (*values.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(0)));
    }
    return Rc::new(RefCell::new(Some(1)));
}