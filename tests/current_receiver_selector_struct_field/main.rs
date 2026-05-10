use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct reader {
    pub path: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct carrier {
    pub path: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


impl carrier {
    pub fn print_reader(&self) {
        let mut r = Rc::new(RefCell::new(Some(reader { path: self.path.clone(), ..Default::default() })));
        println!("{}", (*(*r.borrow().as_ref().unwrap()).path.borrow().as_ref().unwrap()));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(carrier { path: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    (*c.borrow_mut().as_mut().unwrap()).print_reader();
}