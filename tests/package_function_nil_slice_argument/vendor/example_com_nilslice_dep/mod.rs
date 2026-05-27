use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count(values: Rc<RefCell<Option<Vec<i32>>>>) -> i32 {

    if (*values.borrow()).is_none() {
        return 0;
    }
    return 1;
}