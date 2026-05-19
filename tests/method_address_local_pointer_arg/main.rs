use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub ptr: Rc<RefCell<Option<item>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.ptr.borrow().as_ref().unwrap()))
    }
}


impl holder {
    pub fn store(&mut self, ptr: Rc<RefCell<Option<item>>>) {
        { let new_val = ptr.clone(); self.ptr = new_val; };
    }
}

fn main() {
    let mut h = Rc::new(RefCell::new(Some(holder { ptr: Rc::new(RefCell::new(Some(Default::default()))) })));
    let mut value = Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    (*h.borrow_mut().as_mut().unwrap()).store(value.clone());
    println!("{}", (*(*(*h.borrow().as_ref().unwrap()).ptr.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()));
}