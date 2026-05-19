use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct reader {
    pub path: Rc<RefCell<Option<String>>>,
}

impl reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for reader {
    fn default() -> Self {
        Self { path: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct carrier {
    pub path: Rc<RefCell<Option<String>>>,
}

impl carrier {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for carrier {
    fn default() -> Self {
        Self { path: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


impl carrier {
    pub fn print_reader(&self) {
        let mut r = Rc::new(RefCell::new(Some(reader { path: Rc::new(RefCell::new(Some({ let __selector_holder = self.path.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
        println!("{}", (*(*r.borrow().as_ref().unwrap()).path.borrow().as_ref().unwrap()).clone());
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(carrier { path: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    (*c.borrow_mut().as_mut().unwrap()).print_reader();
}