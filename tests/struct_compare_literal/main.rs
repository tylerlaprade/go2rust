use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct version {
    pub major: Rc<RefCell<Option<String>>>,
    pub minor: Rc<RefCell<Option<String>>>,
}

impl version {
    pub fn __go_value_clone(&self) -> Self {
        Self { major: { let __guard = self.major.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, minor: { let __guard = self.minor.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.major.borrow().as_ref().unwrap()), (*self.minor.borrow().as_ref().unwrap()))
    }
}


pub fn parse(x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<version>>> {

    if (*x.borrow().as_ref().unwrap()).clone() == "" {
        return Rc::new(RefCell::new(Some(version { major: Rc::new(RefCell::new(Some(String::new()))), minor: Rc::new(RefCell::new(Some(String::new()))) })));
    }
    return Rc::new(RefCell::new(Some(version { major: x.clone(), minor: Rc::new(RefCell::new(Some("0".to_string()))), ..Default::default() })));
}

pub fn valid(x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*parse(Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()).clone() != version { major: Rc::new(RefCell::new(Some(String::new()))), minor: Rc::new(RefCell::new(Some(String::new()))) })));
}

fn main() {
    println!("{}", (*valid(Rc::new(RefCell::new(Some("1".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*valid(Rc::new(RefCell::new(Some("".to_string())))).borrow().as_ref().unwrap()));
}