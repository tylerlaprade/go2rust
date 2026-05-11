use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct holder {
    pub err: Rc<RefCell<Option<Box<dyn StdError>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { err: self.err.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.err.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut h = Rc::new(RefCell::new(Some(holder { err: Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))), ..Default::default() })));
    println!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*h.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap()));
}