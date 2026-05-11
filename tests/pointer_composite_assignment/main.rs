use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct info {
    pub name: Rc<RefCell<Option<String>>>,
}

impl info {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p: Rc<RefCell<Option<info>>> = Rc::new(RefCell::new(None));
    { let new_val = Rc::new(RefCell::new(Some(info { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }))).clone(); p = new_val; };
    println!("{}", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
}