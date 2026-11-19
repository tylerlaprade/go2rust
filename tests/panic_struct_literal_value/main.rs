use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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
pub struct wrappedErr {
    pub err: Rc<RefCell<Option<Box<dyn StdError>>>>,
    pub tag: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl wrappedErr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = self.tag.clone();
        Self {
            err: __go_clone_0_0,
            tag: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for wrappedErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", match self.tag.borrow().as_ref() { Some(__v) => format_any(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn may_panic(triggerPanic: Rc<RefCell<Option<bool>>>) {
    if (*triggerPanic.borrow().as_ref().unwrap()) {
        panic!("{}", wrappedErr { err: Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))), tag: Rc::new(RefCell::new(Some(Box::new(7) as Box<dyn Any>))), ..Default::default() });
    }
    println!("{}", format!("{}", "no panic".to_string()));
}

fn main() {
    may_panic(Rc::new(RefCell::new(Some(false))));
}