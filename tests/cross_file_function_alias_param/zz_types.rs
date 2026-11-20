use std::cell::{RefCell};
use std::rc::{Rc};

pub type Callback = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>;
