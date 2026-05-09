use crate::aa_use::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub type Callback = Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;
