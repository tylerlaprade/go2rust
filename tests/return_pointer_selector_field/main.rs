use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub child: Rc<RefCell<Option<node>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { child: self.child.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.child.borrow().as_ref().unwrap()))
    }
}


pub fn get_child(h: Rc<RefCell<Option<holder>>>) -> Rc<RefCell<Option<node>>> {
    (*h.borrow().as_ref().unwrap()).child.clone()
}

fn main() {
    if false {
        let mut h = Rc::new(RefCell::new(Some(holder { child: Rc::new(RefCell::new(Some(Default::default()))) })));
        println!("{}", format!("{}", format!("&{}", (*get_child(h.clone()).borrow().as_ref().unwrap()))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}