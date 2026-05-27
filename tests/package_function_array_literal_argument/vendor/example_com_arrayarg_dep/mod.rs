use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Item {
    pub v: Rc<RefCell<Option<i32>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        Self { v: { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Item {
    fn default() -> Self {
        Self { v: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.v.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Item {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("V") {
            out.v = <Rc<RefCell<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub fn of(v: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Item>>> {
    Rc::new(RefCell::new(Some(Item { v: v.clone(), ..Default::default() })))
}

pub fn make(r#static: Rc<RefCell<Option<[Item; 3]>>>, labels: Rc<RefCell<Option<Vec<Item>>>>) -> i32 {
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = r#static.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        { let __rhs = (*item.v.borrow().as_ref().unwrap()); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    { let __range_holder = labels.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        { let __rhs = (*item.v.borrow().as_ref().unwrap()); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    (*total.borrow().as_ref().unwrap())
}