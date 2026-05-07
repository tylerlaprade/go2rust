use std::cell::{RefCell};
use std::rc::{Rc};
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple;

impl std::fmt::Display for types_Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Tuple>")
    }
}


impl types_Tuple {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(Default::default())))
    }
}
