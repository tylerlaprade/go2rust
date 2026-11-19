use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct version {
    pub major: Rc<RefCell<Option<String>>>,
    pub minor: Rc<RefCell<Option<String>>>,
}

impl version {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.major.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.minor.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            major: __go_clone_0_0,
            minor: __go_clone_1_0,
        }
    }
}


impl Default for version {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            major: __go_default_0_0,
            minor: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.major.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.minor.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn parse(x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<version>>> {
    if (*x.borrow().as_ref().unwrap()).clone() == "".to_string() {
        return Rc::new(RefCell::new(Some(version { major: Rc::new(RefCell::new(Some(String::new()))), minor: Rc::new(RefCell::new(Some(String::new()))) })));
    }
    Rc::new(RefCell::new(Some(version { major: Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), minor: Rc::new(RefCell::new(Some("0".to_string()))), ..Default::default() })))
}

pub fn valid(x: Rc<RefCell<Option<String>>>) -> bool {
    (*parse(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).clone() != version { major: Rc::new(RefCell::new(Some(String::new()))), minor: Rc::new(RefCell::new(Some(String::new()))) }
}

fn main() {
    println!("{}", format!("{}", valid(Rc::new(RefCell::new(Some("1".to_string()))))));
    println!("{}", format!("{}", valid(Rc::new(RefCell::new(Some("".to_string()))))));
}