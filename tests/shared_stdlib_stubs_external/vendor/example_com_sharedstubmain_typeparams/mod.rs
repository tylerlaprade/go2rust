use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count() -> i32 {
    let mut tuple = example_com_sharedstubmain_aliases::tuple();
    if (*tuple.borrow()).is_none() {
        return 0;
    }
    (*(*tuple.borrow_mut().as_mut().unwrap()).len().borrow().as_ref().unwrap())
}