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


fn main() {
    let mut r#box: Rc<RefCell<Option<AnonymousStruct1>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut first = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(1 as i32))), ..Default::default() })));
    let mut second = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(2 as i32))), ..Default::default() })));

    { let new_val = first.clone(); (*r#box.borrow_mut().as_mut().unwrap()).child = new_val; };
    { let new_val = second.clone(); (*r#box.borrow_mut().as_mut().unwrap()).child = new_val; };

    println!("{}", format!("{}", (*(*(*r#box.borrow().as_ref().unwrap()).child.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub child: Rc<RefCell<Option<node>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { child: self.child.clone() }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.child.borrow().as_ref().unwrap()))
    }
}
