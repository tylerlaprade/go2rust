use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct customError {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for customError {
    fn default() -> Self {
        Self { msg: Rc::new(RefCell::new(Some(String::new()))) }
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
    if (*err.borrow()).is_none() {
        println!("{}", "nil".to_string());
        return;
    }
    println!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap()));
}

fn main() {
    let mut errs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn StdError>>>>>::from([("one".to_string(), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("one".to_string()))))), ("nil".to_string(), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))]))));
    { let __map_key = "two".to_string(); let __map_value = Rc::new(RefCell::new(Some(Box::new(customError { msg: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() }) as Box<dyn StdError>))); (*errs.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "nil".to_string(); let __map_value = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("three".to_string())))); (*errs.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut err = (*errs.borrow().as_ref().unwrap()).get(&"one".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default());
    if (*err.borrow()).is_some() {
        accept(err.clone());
    }
    accept((*errs.borrow().as_ref().unwrap()).get(&"two".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()));
    accept((*errs.borrow().as_ref().unwrap()).get(&"nil".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()));
}