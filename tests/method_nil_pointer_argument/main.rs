use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct receiver {
}

impl receiver {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl receiver {
    pub fn is_nil(&self, ptr: Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some(true)));
    }
}

fn main() {
    let mut r: Rc<RefCell<Option<receiver>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", (*(*r.borrow().as_ref().unwrap()).is_nil(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap()));
}