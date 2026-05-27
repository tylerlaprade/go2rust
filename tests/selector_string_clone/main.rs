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
        Self { left: { let __guard = self.left.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, right: { let __guard = self.right.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Parts {
    fn default() -> Self {
        Self { left: Rc::new(RefCell::new(Some(String::new()))), right: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.left.borrow().as_ref().unwrap()), (*self.right.borrow().as_ref().unwrap()))
    }
}


pub fn left_of(parts: Rc<RefCell<Option<Parts>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __selector_holder = (*parts.borrow().as_ref().unwrap()).left.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))
}

pub fn echo(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(value.borrow().as_ref().unwrap().clone())))
}

fn main() {
    let mut parts = Rc::new(RefCell::new(Some(Parts { left: Rc::new(RefCell::new(Some("go".to_string()))), right: Rc::new(RefCell::new(Some("rust".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*left_of(Rc::new(RefCell::new(Some((*parts.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*echo({ let __field = (*parts.borrow().as_ref().unwrap()).right.clone(); __field }).borrow().as_ref().unwrap())));
}