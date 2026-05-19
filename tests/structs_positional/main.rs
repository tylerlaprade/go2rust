use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Pair {
    pub left: Rc<RefCell<Option<String>>>,
    pub right: Rc<RefCell<Option<i32>>>,
}

impl Pair {
    pub fn __go_value_clone(&self) -> Self {
        Self { left: { let __guard = self.left.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, right: { let __guard = self.right.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Pair {
    fn default() -> Self {
        Self { left: Rc::new(RefCell::new(Some(String::new()))), right: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.left.borrow().as_ref().unwrap()), (*self.right.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(Pair { left: Rc::new(RefCell::new(Some("go".to_string()))), right: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    println!("{} {}", format!("{}", (*(*p.borrow().as_ref().unwrap()).left.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*p.borrow().as_ref().unwrap()).right.borrow().as_ref().unwrap())));
}