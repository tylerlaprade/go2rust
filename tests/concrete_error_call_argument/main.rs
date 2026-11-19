use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct customError {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            msg: __go_clone_0_0,
        }
    }
}


impl Default for customError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            msg: __go_default_0_0,
        }
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


pub fn accept(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}

fn main() {
    accept(Rc::new(RefCell::new(Some(Rc::new(customError { msg: Rc::new(RefCell::new(Some("boom".to_string()))), ..Default::default() }) as Rc<dyn StdError>))));
}