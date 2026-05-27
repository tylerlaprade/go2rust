use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub fn has_name(it: Rc<RefCell<Option<item>>>) -> bool {

    return (*it.borrow()).is_some() && { let __selector_holder = (*it.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == "ready";
}

pub fn missing_or_ready(it: Rc<RefCell<Option<item>>>) -> bool {

    return (*it.borrow()).is_none() || { let __selector_holder = (*it.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == "ready";
}

fn main() {
    println!("{}", format!("{}", has_name(Rc::new(RefCell::new(None)))));
    println!("{}", format!("{}", has_name(Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }))))));
    println!("{}", format!("{}", missing_or_ready(Rc::new(RefCell::new(None)))));
    println!("{}", format!("{}", missing_or_ready(Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("other".to_string()))), ..Default::default() }))))));
}