use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn count() -> Rc<RefCell<Option<i32>>> {

    let mut tuple = example_com_sharedstubmain_aliases::tuple();
    if (*tuple.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(0 as i32)));
    }
    return (*tuple.borrow_mut().as_mut().unwrap()).len();
}