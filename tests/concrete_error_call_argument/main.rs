use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct customError {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for customError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl customError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return self.msg.clone();
    }
}

impl StdError for customError {}


pub fn accept(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    println!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap()));
}

fn main() {
    accept(Rc::new(RefCell::new(Some(Box::new(customError { msg: Rc::new(RefCell::new(Some("boom".to_string()))), ..Default::default() }) as Box<dyn StdError>))));
}