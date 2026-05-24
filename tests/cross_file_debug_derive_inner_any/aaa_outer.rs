use crate::{format_any};

use crate::bbb_inner::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone)]
pub struct Outer {
    pub name: Rc<RefCell<Option<String>>>,
    pub inner: Rc<RefCell<Option<Inner>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, inner: self.inner.clone() }
    }
}


impl Default for Outer {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), inner: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.inner.borrow().as_ref().unwrap()))
    }
}
