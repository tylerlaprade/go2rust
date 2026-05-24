use crate::{format_any};

use crate::aaa_outer::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone)]
pub struct Inner {
    pub tag: Rc<RefCell<Option<String>>>,
    pub data: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Inner {
    pub fn __go_value_clone(&self) -> Self {
        Self { tag: { let __guard = self.tag.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for Inner {
    fn default() -> Self {
        Self { tag: Rc::new(RefCell::new(Some(String::new()))), data: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tag.borrow().as_ref().unwrap()), format_any(self.data.borrow().as_ref().unwrap().as_ref()))
    }
}
