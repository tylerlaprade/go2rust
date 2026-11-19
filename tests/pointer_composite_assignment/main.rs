use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct info {
    pub name: Rc<RefCell<Option<String>>>,
}

impl info {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for info {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut p: Rc<RefCell<Option<info>>> = Rc::new(RefCell::new(None));
    { let new_val = Rc::new(RefCell::new(Some(info { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }))).clone(); p = new_val; };
    println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}