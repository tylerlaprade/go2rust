use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Clone, Default)]
pub struct entry {
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_any(self.value.borrow().as_ref().unwrap().as_ref()))
    }
}


#[derive(Clone, Default)]
pub struct holder {
    pub table: Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<Vec<entry>>>>>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { table: self.table.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", "<map>")
    }
}


fn main() {
    if false {
        println!("{}", holder { table: Rc::new(RefCell::new(Some(BTreeMap::new()))) });
    }
    println!("{}", "ok".to_string());
}