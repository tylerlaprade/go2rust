use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub n: Rc<RefCell<Option<atomic_Int32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Counter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Counter {
    pub fn add(&self, delta: Rc<RefCell<Option<i32>>>) -> i32 {
        return (*(*self.n.borrow_mut().as_mut().unwrap()).add(delta.clone()).borrow().as_ref().unwrap());
    }
}

pub fn new_counter() -> Rc<RefCell<Option<Counter>>> {

    return Rc::new(RefCell::new(Some(Counter { n: Rc::new(RefCell::new(Some(Default::default()))) })));
}