use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn has_name(it: Rc<RefCell<Option<item>>>) -> bool {
    return { let __nil_result = (*it.borrow()).is_some(); __nil_result } && { let __selector_holder = (*it.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == "ready".to_string();
}

pub fn missing_or_ready(it: Rc<RefCell<Option<item>>>) -> bool {
    return { let __nil_result = (*it.borrow()).is_none(); __nil_result } || { let __selector_holder = (*it.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == "ready".to_string();
}

fn main() {
    println!("{}", format!("{}", has_name(Rc::new(RefCell::new(None)))));
    println!("{}", format!("{}", has_name(Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }))))));
    println!("{}", format!("{}", missing_or_ready(Rc::new(RefCell::new(None)))));
    println!("{}", format!("{}", missing_or_ready(Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("other".to_string()))), ..Default::default() }))))));
}