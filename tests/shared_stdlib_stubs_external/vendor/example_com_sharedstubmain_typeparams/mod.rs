use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count() -> i32 {
    let mut tuple = example_com_sharedstubmain_aliases::tuple();
    if { let __nil_result = (*tuple.borrow()).is_none(); __nil_result } {
        return 0;
    }
    return (*tuple.borrow_mut().as_mut().unwrap()).len();
}