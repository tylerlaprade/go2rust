use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct wrappedError {
    pub error: Rc<RefCell<Option<Box<dyn StdError>>>>,
}

impl wrappedError {
    pub fn __go_value_clone(&self) -> Self {
        Self { error: self.error.clone() }
    }
}

impl std::fmt::Display for wrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for wrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


impl wrappedError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}", (*self.error.borrow().as_ref().unwrap())))))
    }
}

impl StdError for wrappedError {}


pub fn build() -> Rc<RefCell<Option<Box<dyn StdError>>>> {

    return Rc::new(RefCell::new(Some(Box::new(wrappedError { error: Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("wrapped {}", "boom".to_string()))))), ..Default::default() }) as Box<dyn StdError>)));
}

fn main() {
    let mut err = build();
    println!("{}", format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
}