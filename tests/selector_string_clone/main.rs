use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Parts {
    pub left: Rc<RefCell<Option<String>>>,
    pub right: Rc<RefCell<Option<String>>>,
}

impl Parts {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.left.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.right.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            left: __go_clone_0_0,
            right: __go_clone_1_0,
        }
    }
}


impl Default for Parts {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            left: __go_default_0_0,
            right: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.left.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.right.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn left_of(parts: Rc<RefCell<Option<Parts>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*parts.borrow().as_ref().unwrap()).left.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

pub fn echo(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    { let __owned = value.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

fn main() {
    let mut parts = Rc::new(RefCell::new(Some(Parts { left: Rc::new(RefCell::new(Some("go".to_string()))), right: Rc::new(RefCell::new(Some("rust".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*left_of(Rc::new(RefCell::new(Some({ let __arg_holder = parts.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*echo(Rc::new(RefCell::new(Some({ let __selector_holder = (*parts.borrow().as_ref().unwrap()).right.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).borrow().as_ref().unwrap())));
}