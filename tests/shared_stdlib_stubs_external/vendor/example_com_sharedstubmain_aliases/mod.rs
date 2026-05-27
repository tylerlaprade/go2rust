use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn tuple() -> Rc<RefCell<Option<types_Tuple>>> {
    Rc::new(RefCell::new(None))
}