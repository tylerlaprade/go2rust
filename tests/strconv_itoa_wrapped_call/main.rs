use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub index: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: { let __guard = self.index.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { index: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.index.borrow().as_ref().unwrap()))
    }
}


impl item {
    pub fn index(&self) -> Rc<RefCell<Option<i32>>> {
        return self.index.clone();
    }
}

fn main() {
    let mut v = Rc::new(RefCell::new(Some(item { index: Rc::new(RefCell::new(Some(3 as i32))), ..Default::default() })));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some((*(*v.borrow().as_ref().unwrap()).index().borrow().as_ref().unwrap()).to_string()))).borrow().as_ref().unwrap())));
}