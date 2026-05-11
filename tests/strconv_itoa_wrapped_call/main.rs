use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub index: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: { let __guard = self.index.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
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
    let mut v = Rc::new(RefCell::new(Some(item { index: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    println!("{}", (*Rc::new(RefCell::new(Some((*(*v.borrow().as_ref().unwrap()).index().borrow().as_ref().unwrap()).to_string()))).borrow().as_ref().unwrap()));
}