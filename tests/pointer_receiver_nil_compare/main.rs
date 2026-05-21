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


impl node {
    pub fn same(&self, other: Rc<RefCell<Option<node>>>) -> Rc<RefCell<Option<bool>>> {
        if false || (*other.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(false)));
    }
        return Rc::new(RefCell::new(Some((*self.value.borrow().as_ref().unwrap()) == (*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()))));
    }
}

fn main() {
    let mut left = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() })));
    let mut missing: Rc<RefCell<Option<node>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", (*(*left.borrow().as_ref().unwrap()).same(missing.clone()).borrow().as_ref().unwrap())));
}