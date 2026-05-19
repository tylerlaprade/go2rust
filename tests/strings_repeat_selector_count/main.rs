use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct printer {
    pub indent: Rc<RefCell<Option<i32>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        Self { indent: { let __guard = self.indent.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for printer {
    fn default() -> Self {
        Self { indent: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.indent.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(printer { indent: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}{}", (*Rc::new(RefCell::new(Some({ let __s = "..".to_string(); let __count = (*(*p.borrow().as_ref().unwrap()).indent.borrow().as_ref().unwrap()); __s.repeat(__count as usize) }))).borrow().as_ref().unwrap()), "x".to_string()));
}