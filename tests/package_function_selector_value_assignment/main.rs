use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub mod log {
    use super::*;
    pub fn printf<T0, T1>(_arg0: T0, _arg1: T1) {
    }
}


#[derive(Clone, Default)]
pub struct Config {
    pub logf: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        Self { logf: self.logf.clone() }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", "<func>")
    }
}


#[derive(Clone)]
pub struct loader {
    pub config: Rc<RefCell<Option<Config>>>,
}

impl loader {
    pub fn __go_value_clone(&self) -> Self {
        Self { config: { let __guard = self.config.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for loader {
    fn default() -> Self {
        Self { config: Rc::new(RefCell::new(Some(Config::default()))) }
    }
}

impl std::fmt::Display for loader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.config.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut cfg: Rc<RefCell<Option<Config>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| { log::printf(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))); (*cfg.borrow_mut().as_mut().unwrap()).logf = new_val; };
    let mut copied: Rc<RefCell<Option<Config>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = (*cfg.borrow().as_ref().unwrap()).logf.clone(); (*copied.borrow_mut().as_mut().unwrap()).logf = new_val; };
    let mut ld: Rc<RefCell<Option<loader>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = (*cfg.borrow().as_ref().unwrap()).logf.clone(); (*(*ld.borrow().as_ref().unwrap()).config.borrow_mut().as_mut().unwrap()).logf = new_val; };
    { let new_val = Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| { log::printf(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))); (*(*ld.borrow().as_ref().unwrap()).config.borrow_mut().as_mut().unwrap()).logf = new_val; };
    println!("{}", format!("{}", "assigned".to_string()));
}