use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Counter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Value") {
            out.value = <Rc<RefCell<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl Counter {
    pub fn total(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.value.borrow().as_ref().unwrap()) + (*self.value.borrow().as_ref().unwrap()))));
    }
}

pub fn new_counter() -> Rc<RefCell<Option<Counter>>> {

    return Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
}