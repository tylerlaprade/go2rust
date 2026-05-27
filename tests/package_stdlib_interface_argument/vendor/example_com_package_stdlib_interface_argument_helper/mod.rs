use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn r#use(r: Rc<RefCell<Option<io_Reader>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("reader".to_string())))
}